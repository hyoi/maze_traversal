use super::*;

//Pluginの手続き
pub struct PluginInitialize;
impl Plugin for PluginInitialize
{	fn build( &self, app: &mut AppBuilder )
	{	app
		//------------------------------------------------------------------------------------------
		.add_system_set											//GameState::Initialize
		(	SystemSet::on_enter( GameState::Initialize )		// on_enter()
				.with_system( start_preload_assets.system() )	// Assetのロード開始
				.with_system( spawn_ui_invisible.system() )		// UI(メッセージ)を非表示で生成
		)
		.add_system_set											//GameState::Initialize
		(	SystemSet::on_update( GameState::Initialize )		// on_update()
				.with_system( goto_next_after_loaded.system() )	// ロード完了⇒GameStartへ遷移
		)
		//------------------------------------------------------------------------------------------
		;
	}
}

////////////////////////////////////////////////////////////////////////////////////////////////////

//定義と定数

//プレー開始前にロードするAssetと、ロード済みAssetのハンドルを保管するResource
const COUNTDOWN_FONT_FILE : &str = "fonts/Orbitron-Black.ttf";
pub const SPRITE_WALL_FILE: &str = "sprites/wall.png";

const PRELOAD_ASSET_FILES: [ &str; 2 ] =
[	COUNTDOWN_FONT_FILE,
	SPRITE_WALL_FILE,
];

struct LoadedHandles ( Vec<HandleUntyped> );

//Text UIの情報をまとめた型エイリアス
type MessageSect<'a> = ( &'a str, &'a str, f32, Color );

//GameClearのメッセージ（カウントダウン表示枠付き）
pub struct MessageClear;
const MESSAGE_CLEAR: [ MessageSect; 3 ] =
[	( "Clear!!\n\n"      , COUNTDOWN_FONT_FILE, 3., Color::GOLD ),
	( "Next stage...\n\n", COUNTDOWN_FONT_FILE, 2., Color::GOLD ),
	( ""                 , COUNTDOWN_FONT_FILE, 4., Color::GOLD ),
];

//Pauseのメッセージ
pub struct MessagePause;
const MESSAGE_PAUSE: [ MessageSect; 1 ] =
[	( "P A U S E", COUNTDOWN_FONT_FILE, 5., Color::WHITE ),
];

////////////////////////////////////////////////////////////////////////////////////////////////////

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

////////////////////////////////////////////////////////////////////////////////////////////////////

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