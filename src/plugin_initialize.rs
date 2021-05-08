use super::*;

//Plugin
pub struct PluginInitialize;

impl Plugin for PluginInitialize
{	fn build( &self, app: &mut AppBuilder )
	{	app
		//--------------------------------------------------------------------------------
			.add_state( GameState::Initialize )					// 状態遷移のState初期値
			.add_event::<GameState>()							// 状態遷移のEventキュー
		//--------------------------------------------------------------------------------
			.init_resource::<GameRecord>()						// ゲームレコード
			.init_resource::<GameStage>()						// ゲームステージ
		//--------------------------------------------------------------------------------
			.add_startup_system( spawn_camera.system() )		// bevyのカメラ設置
			.add_system( judge_pause_key_input.system() )		// pause処理
			.add_system( egui_window.system() )					// ステージ数とスコアの表示
		//--------------------------------------------------------------------------------
			//GameState::Initialize
			.add_system_set
			(	SystemSet::on_enter( GameState::Initialize )	// on_enter()
				.with_system( start_preload_assets.system() )	// Assetのロード開始
				.with_system( spawn_ui_invisible.system() )		// UI(メッセージ)を非表示で生成
			)
			.add_system_set
			(	SystemSet::on_update( GameState::Initialize )	// on_update()
				.with_system( goto_next_after_loaded.system() )	// ロード完了⇒GameStartへ遷移
			)
		;

		//================================================================================
		#[cfg(not(target_arch = "wasm32"))]
		app.add_system( toggle_window_mode.system() );			// [Alt]+[Enter]でフルスクリーン
		//================================================================================
	}
}

////////////////////////////////////////////////////////////////////////////////

//ゲームレコード
#[derive(Default)]
pub struct GameRecord
{	pub score: usize,
}

//ゲームステージ（Defaultはmap.rsで定義）
pub struct GameStage
{	pub rng: rand::prelude::StdRng,	//再現性がある乱数を使いたいので
	pub level: usize,
	pub map: [ [ MapObj; MAP_HEIGHT ]; MAP_WIDTH ],
	pub count_dots: usize,
	pub start_xy: ( usize, usize ),
	pub goal_xy : ( usize, usize ),
}

////////////////////////////////////////////////////////////////////////////////

//bevyのカメラの設置
fn spawn_camera( mut cmds: Commands )
{	cmds.spawn_bundle( UiCameraBundle::default() );
	cmds.spawn_bundle( OrthographicCameraBundle::new_2d() );
}

//[Esc]が入力さたらPauseする
fn judge_pause_key_input
(	mut q_ui : Query<&mut Visible, With<MessagePause>>,
	mut state: ResMut<State<GameState>>,
	mut inkey: ResMut<Input<KeyCode>>,
)
{	let pause_key = KeyCode::Escape;
	if let Ok( mut ui ) = q_ui.single_mut()
	{	if inkey.just_pressed( pause_key ) 
		{	match state.current()
			{	GameState::Pause => { ui.is_visible = false; state.pop().unwrap() },
				_                => { ui.is_visible = true ; state.push( GameState::Pause ).unwrap() },
			};
			inkey.reset( pause_key ); // https://bevy-cheatbook.github.io/programming/states.html#with-input
		}
	}
}

//[Alt]+[Enter]でウィンドウとフルスクリーンを切り替える
#[cfg(not(target_arch = "wasm32"))]
fn toggle_window_mode( mut window: ResMut<Windows>, inkey: Res<Input<KeyCode>> )
{	use KeyCode::*;
	let is_alt = inkey.pressed( LAlt ) || inkey.pressed( RAlt );
	let is_alt_return = is_alt && inkey.just_pressed( Return );

	if is_alt_return
	{	use bevy::window::WindowMode::*;
		if let Some( window ) = window.get_primary_mut()
		{	let mode = if window.mode() == Windowed
				{ Fullscreen { use_size: true } } else { Windowed };
			window.set_mode( mode );
		}
	}
}

//ステージ数とスコアの表示
fn egui_window( egui: Res<EguiContext>, stage: Res<GameStage>, record: Res<GameRecord> )
{	egui::Window::new( APP_TITLE ).show
	(	egui.ctx(),
		|ui| { ui.label( format!( "Stage: {}\nScore: {}", stage.level, record.score ) ); }
	);
}

////////////////////////////////////////////////////////////////////////////////

//プレー開始前にロードするAssetと、ロード済みAssetのハンドルを保管するResource
const COUNTDOWN_FONT_FILE : &str = "fonts/Orbitron-Black.ttf";
pub const SPRITE_WALL_FILE: &str = "sprites/wall.png";

const PRELOAD_ASSET_FILES: [ &str; 2 ] =
[	COUNTDOWN_FONT_FILE,
	SPRITE_WALL_FILE,
];

struct LoadedHandles ( Vec<HandleUntyped> );

//Assetの事前ロードを開始する
fn start_preload_assets( mut cmds: Commands, server: Res<AssetServer> )
{	//Assetのロードを開始
	let mut preload = Vec::new();
	PRELOAD_ASSET_FILES.iter()
		.for_each( | file | preload.push( server.load_untyped( *file ) ) );

	cmds.insert_resource( LoadedHandles ( preload ) );
}

//Assetのロードが完了したら、Stateを変更
fn goto_next_after_loaded
(	mut state: ResMut<State<GameState>>,
	preload: Res<LoadedHandles>,
	server: Res<AssetServer>,
)
{	for handle in preload.0.iter()
	{	match server.get_load_state( handle )
		{	bevy::asset::LoadState::Loaded => {}
			bevy::asset::LoadState::Failed => { panic!() } //ロードエラー⇒パニック
			_  => { return } //ロード完了待ち
		}
	}

	//GameStartへ遷移する
	state.overwrite_set( GameState::GameStart ).unwrap();
}

////////////////////////////////////////////////////////////////////////////////

//Text UIの情報をまとめた型エイリアス
type MessageSect<'a> = ( &'a str, &'a str, f32, Color );

//GameClearのメッセージ（カウントダウン表示枠付き）
const MESSAGE_CLEAR: [ MessageSect; 3 ] =
[	( "Clear!!\n\n"      , COUNTDOWN_FONT_FILE, 3., Color::GOLD ),
	( "Next stage...\n\n", COUNTDOWN_FONT_FILE, 2., Color::GOLD ),
	( ""                 , COUNTDOWN_FONT_FILE, 4., Color::GOLD ),
];

//Pauseのメッセージ
const MESSAGE_PAUSE: [ MessageSect; 1 ] =
[	( "P A U S E", COUNTDOWN_FONT_FILE, 5., Color::WHITE ),
];

//メッセージ用のテキストのComponent
pub struct MessageClear;
struct MessagePause;

//メッセージ用のテキストを非表示で配置する
fn spawn_ui_invisible( mut cmds: Commands, asset_svr: Res<AssetServer> )
{	//メッセージの準備
	let mut clear = text_messsage( &MESSAGE_CLEAR, &asset_svr );
	let mut pause = text_messsage( &MESSAGE_PAUSE, &asset_svr );
	clear.visible.is_visible = false;
	pause.visible.is_visible = false;

	//隠しフレームの上に子要素を作成する
	cmds.spawn_bundle( hidden_frame_full_screen() ).with_children
	(	| cmds |
		{	cmds.spawn_bundle( clear ).insert( MessageClear );
			cmds.spawn_bundle( pause ).insert( MessagePause );
		}
	);
}

//TextBundleを作る
fn text_messsage( message: &[ MessageSect ], asset_svr: &Res<AssetServer> ) -> TextBundle
{	//TextSectionのVecを作る
	let mut sections = Vec::new();
	for ( mess, font, size, color ) in message.iter()
	{	let section = TextSection
		{	value: mess.to_string(),
			style: TextStyle
			{	font     : asset_svr.load( *font ),
				font_size: PIXEL_PER_GRID * size,
				color    : *color,
			}
		};
		sections.push( section );
	}

	//バンドルを返す
	TextBundle
	{	style: Style
		{	position_type: PositionType::Absolute,
			..Default::default()
		},
		text: Text
		{	sections,
			alignment: TextAlignment
			{	vertical  : VerticalAlign::Center,
				horizontal: HorizontalAlign::Center,
			}
		},
		..Default::default()
	}
}

//フルスクリーンの画面サイズに対し縦横100%の隠しフレーム
fn hidden_frame_full_screen() -> NodeBundle
{	let per100 = Val::Percent( 100. );
	NodeBundle
	{	style: Style
		{	size: Size::new( per100, per100 ),
			position_type  : PositionType::Absolute,
			justify_content: JustifyContent::Center,
			align_items    : AlignItems::Center,
			..Default::default()
		},
		visible: Visible { is_visible: false, ..Default::default() },
		..Default::default()
	}
}

//End of code.