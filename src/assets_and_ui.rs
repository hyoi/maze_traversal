use super::*;

//Pluginの手続き
pub struct PluginInit;
impl Plugin for PluginInit
{	fn build( &self, app: &mut AppBuilder )
	{	app
		//------------------------------------------------------------------------------------------
		.add_system_set													// GameState::Init
		(	SystemSet::on_enter( GameState::Init )						// on_enter()
				.with_system( start_preload_assets.system() )			// Assetのロード開始
		)
		.add_system_set													// GameState::Init
		(	SystemSet::on_update( GameState::Init )						// on_update()
				.with_system( change_state_after_loading.system() )		// ロード完了⇒GameState::Startへ
		)
		.add_system_set													// GameState::Init
		(	SystemSet::on_exit( GameState::Init )						// on_exit()
				.with_system( spawn_text_ui_message.system() )			// UIを非表示で生成
		)
		//------------------------------------------------------------------------------------------
		.add_system_set													// GameState::Clear
		(	SystemSet::on_enter( GameState::Clear )						// on_enter()
				.with_system( show_clear_message.system() )				// CLEARメッセージを表示する
		)
		.add_system_set													// GameState::Clear
		(	SystemSet::on_update( GameState::Clear )					// on_update()
				.with_system( change_state_after_countdown.system() )	// CD完了⇒GameState::Startへ
		)
		.add_system_set													// GameState::Clear
		(	SystemSet::on_exit( GameState::Clear )						// on_exit()
				.with_system( hide_clear_message.system() )				// CLEARメッセージを隠す
		)
		//------------------------------------------------------------------------------------------
		;
	}
}

////////////////////////////////////////////////////////////////////////////////////////////////////

//定義と定数

//Assetsのプリロードとハンドルの保存
const CENTER_TEXT_FONT: &str = "fonts/Orbitron-Black.ttf";

const PRELOAD_ASSET_FILES: [ &str; 2 ] =
[	CENTER_TEXT_FONT,
	WALL_SPRITE_FILE,	//定義はmap.rs
];

struct LoadedAssets { preload: Vec<HandleUntyped> }

//Text UI
type MessageSect<'a> = ( &'a str, &'a str, f32, Color );

pub struct MessagePause;
const MESSAGE_PAUSE: [ MessageSect; 1 ] =
[	( "P A U S E", CENTER_TEXT_FONT, PIXEL_PER_GRID * 5.0, Color::SILVER ),
];

pub struct MessageClear;
const MESSAGE_CLEAR: [ MessageSect; 3 ] =
[	( "C L E A R !!\n"   , CENTER_TEXT_FONT, PIXEL_PER_GRID * 5.0, Color::GOLD  ),
	( "Next stage...\n\n", CENTER_TEXT_FONT, PIXEL_PER_GRID * 2.0, Color::WHITE ),
	( ""                 , CENTER_TEXT_FONT, PIXEL_PER_GRID * 4.0, Color::WHITE ),
];

//CountdownTimer
#[derive(Default)]
struct CountDown { timer: Timer }

////////////////////////////////////////////////////////////////////////////////////////////////////

//Assetの事前ロードを開始する
fn start_preload_assets
(	mut cmds: Commands,
	asset_svr: Res<AssetServer>,
)
{	//Assetのロードを開始
	let mut preload = Vec::new();
	PRELOAD_ASSET_FILES.iter()
		.for_each( | f | preload.push( asset_svr.load_untyped( *f ) ) );

	cmds.insert_resource( LoadedAssets { preload } );
}

//Assetのロードが完了したら、Stateを変更
fn change_state_after_loading
(	mut state : ResMut<State<GameState>>,
	assets: Res<LoadedAssets>,
	asset_svr: Res<AssetServer>,
)
{	for handle in assets.preload.iter()
	{	use bevy::asset::LoadState::*;
		match asset_svr.get_load_state( handle )
		{	Loaded => {}
			Failed => panic!(),	//ロードエラー⇒パニック
			_      => return,	//on_update()なので繰り返し関数が呼び出される
		}
	}

	//Startへ遷移する
	let _ = state.overwrite_set( GameState::Start );
}

////////////////////////////////////////////////////////////////////////////////////////////////////

//テキストUIを配置する
fn spawn_text_ui_message( mut cmds: Commands, asset_svr: Res<AssetServer> )
{	//中央に表示するtext
	let mut pause_text = text_messsage( &MESSAGE_PAUSE, &asset_svr );
	let mut clear_text = text_messsage( &MESSAGE_CLEAR, &asset_svr );
	pause_text.visible.is_visible = false;	//初期は非表示
	clear_text.visible.is_visible = false;	//初期は非表示

	//隠しフレームの上に子要素を作成する
	cmds.spawn_bundle( hidden_frame_for_centering() ).with_children( | cmds |
	{	cmds.spawn_bundle( pause_text ).insert( MessagePause );
		cmds.spawn_bundle( clear_text ).insert( MessageClear );
	} );
}

//TextBundleを作る
fn text_messsage( message: &[ MessageSect ], asset_svr: &Res<AssetServer> ) -> TextBundle
{	let mut sections = Vec::new();
	for ( value, file, size, color ) in message.iter()
	{	let value = value.to_string();
		let style = TextStyle
		{	font     : asset_svr.load( *file ),
			font_size: *size,
			color    : *color
		};
		sections.push( TextSection { value, style } );
	}
	let alignment = TextAlignment { vertical: VerticalAlign::Center, horizontal: HorizontalAlign::Center };
	let text = Text { sections, alignment };
	let style = Style { position_type: PositionType::Absolute, ..Default::default() };
	TextBundle { style, text, ..Default::default() }
}

//中央寄せ用の隠しフレーム
fn hidden_frame_for_centering() -> NodeBundle
{	let per100 = Val::Percent( 100.0 );
	let style = Style
	{	size: Size::new( per100, per100 ),
		position_type  : PositionType::Absolute,
		justify_content: JustifyContent::Center,
		align_items    : AlignItems::Center,
		..Default::default()
	};
	let visible = Visible { is_visible: false, ..Default::default() };
	NodeBundle { style, visible, ..Default::default() }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

//CLEARメッセージ表示
fn show_clear_message( mut q: Query<&mut Visible, With<MessageClear>> )
{	if let Ok( mut ui ) = q.single_mut() { ui.is_visible = true }
}

//CLEARメッセージ非表示
fn hide_clear_message( mut q: Query<&mut Visible, With<MessageClear>> )
{	if let Ok( mut ui ) = q.single_mut() { ui.is_visible = false }
}

//カウントダウンの後、Startへ遷移
fn change_state_after_countdown
(	mut q: Query<&mut Text, With<MessageClear>>,
	mut state: ResMut<State<GameState>>,
	( mut count, mut countdown ): ( Local<i32>, Local<CountDown> ),
	time: Res<Time>,
)
{	if let Ok( mut ui ) = q.single_mut()
	{	if *count <= 0											 //カウンターが未初期化なら
		{	countdown.timer = Timer::from_seconds( 1.0, false ); //タイマーセット
			*count = 4;											 //カウンター初期化
		}
		else if countdown.timer.tick( time.delta() ).finished()	 //1秒経過したら
		{	countdown.timer.reset();							 //タイマー再セット
			*count -= 1;										 //カウントダウン

			//カウントダウンが終わったら、Startへ遷移する
			if *count <= 0 { let _ = state.overwrite_set( GameState::Start ); }
		}
		ui.sections[ 2 ].value = format!( "{}", ( *count - 1 ).max( 0 ) );
	}
}

//End of code.