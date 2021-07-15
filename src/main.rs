//external modules
use bevy::{ prelude::*, diagnostic::*,};
use bevy_prototype_lyon::{ prelude::*, entity::ShapeBundle };
use bevy_egui::*;
use rand::prelude::*;

//internal modules
mod ui;
mod map;
mod player;
mod control_panel;
use ui::*;
use map::*;
use player::*;
use control_panel::*;

////////////////////////////////////////////////////////////////////////////////////////////////////

//アプリのTitle
const APP_TITLE: &str = "maze traversal";

//表示倍率、ウィンドウの縦横pixel数と背景色
const SCREEN_SCALING: usize = 3;
const PIXEL_PER_GRID: f32   = ( 8 * SCREEN_SCALING ) as f32;
const SCREEN_WIDTH  : f32   = PIXEL_PER_GRID * MAP_WIDTH  as f32;
const SCREEN_HEIGHT : f32   = PIXEL_PER_GRID * ( MAP_HEIGHT as f32 + 2.0 );
const SCREEN_BGCOLOR: Color = Color::rgb_linear( 0.025, 0.025, 0.04 );

//ゲームの状態遷移
#[derive(Clone,Copy,Debug,Eq,PartialEq,Hash)]
pub enum GameState { Init, Start, Play, Clear, Pause }

//全体に影響する変数を格納するResource
struct SystemParameters
{	maze_type: SelectMazeType,
}
impl Default for SystemParameters
{	fn default() -> Self
	{	Self
		{	maze_type: SelectMazeType::Random
		}
	}
}

////////////////////////////////////////////////////////////////////////////////////////////////////

//メイン関数
fn main()
{	let main_window = WindowDescriptor
	{	title    : APP_TITLE.to_string(),
		width    : SCREEN_WIDTH,
		height   : SCREEN_HEIGHT,
		resizable: false,
		..Default::default()
	};
	
	let mut app = App::build();
	app
	//----------------------------------------------------------------------------------------------
	.insert_resource( main_window )									// メインウィンドウ
	.insert_resource( ClearColor( SCREEN_BGCOLOR ) )				// 背景色
	.insert_resource( Msaa { samples: 4 } )							// アンチエイリアス
	//----------------------------------------------------------------------------------------------
	.add_plugins( DefaultPlugins )									// デフォルトプラグイン
	.add_plugin( FrameTimeDiagnosticsPlugin::default() )			// fps計測のプラグイン
	.add_plugin( ShapePlugin )										// bevy_prototype_lyon
	.add_plugin( EguiPlugin )										// bevy_egui
	//----------------------------------------------------------------------------------------------
	.add_state( GameState::Init )									// 状態遷移の初期値
	.init_resource::<SystemParameters>()							// 全体に影響する変数を格納するResource
	.add_system_set													// ＜GameState::Init＞
	(	SystemSet::on_enter( GameState::Init )						// ＜on_enter()＞
			.with_system( start_preload_assets.system() )			// Assetの事前ロード開始
	)
	.add_system_set													// ＜GameState::Init＞
	(	SystemSet::on_update( GameState::Init )						// ＜on_update()＞
			.with_system( change_state_after_loading.system() )		// ロード完了⇒GameState::Startへ
	)
	//----------------------------------------------------------------------------------------------
	.add_startup_system( spawn_camera.system() )					// bevyのカメラ設置
	.add_system( handle_esc_key_for_pause.system() )				// [Esc]でpause処理
	//----------------------------------------------------------------------------------------------
	.add_plugin( PluginUi )
	.add_plugin( PluginControlPanel )
	.add_plugin( PluginMap )
	.add_plugin( PluginPlayer )
	//----------------------------------------------------------------------------------------------
	;

	#[cfg(not(target_arch = "wasm32"))]
	//----------------------------------------------------------------------------------------------
	app.add_system( toggle_window_mode.system() );					// [Alt]+[Enter]でフルスクリーン
	//----------------------------------------------------------------------------------------------

	#[cfg(target_arch = "wasm32")]
	//----------------------------------------------------------------------------------------------
	app.add_plugin( bevy_webgl2::WebGL2Plugin );					// WASM用のプラグイン
	//----------------------------------------------------------------------------------------------

	app.run();														// アプリの実行
}

////////////////////////////////////////////////////////////////////////////////////////////////////

//Assetsのプリロードとハンドルの保存
const PRELOAD_ASSET_FILES: [ &str; 3 ] =
[	FONT_MESSAGE_TEXT,	//定義はui.rs
	FONT_TITLE_TEXT, 	//定義はui.rs
	WALL_SPRITE_FILE,	//定義はmap.rs
];
struct LoadedAssets { preload: Vec<HandleUntyped> }

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

//bevyのカメラの設置
fn spawn_camera( mut cmds: Commands )
{	cmds.spawn_bundle( UiCameraBundle::default() );
	cmds.spawn_bundle( OrthographicCameraBundle::new_2d() );
}

////////////////////////////////////////////////////////////////////////////////////////////////////

//[Alt]+[Enter]でウィンドウとフルスクリーンを切り替える
#[cfg(not(target_arch = "wasm32"))]
fn toggle_window_mode( inkey: Res<Input<KeyCode>>, mut window: ResMut<Windows> )
{	use KeyCode::*;
	let is_alt = inkey.pressed( LAlt ) || inkey.pressed( RAlt );
	let is_alt_return = is_alt && inkey.just_pressed( Return );

	if is_alt_return
	{	use bevy::window::WindowMode::*;
		if let Some( window ) = window.get_primary_mut()
		{	let mode = if window.mode() == Windowed { Fullscreen { use_size: true } } else { Windowed };
			window.set_mode( mode );
		}
	}
}

//[Esc]が入力さたらPauseする
fn handle_esc_key_for_pause
(	mut q: Query<&mut Visible, With<MessagePause>>,
	mut state: ResMut<State<GameState>>,
	mut inkey: ResMut<Input<KeyCode>>,
)
{	if ! inkey.just_pressed( KeyCode::Escape ) { return }

	let now = *state.current();
	if now != GameState::Pause && now != GameState::Play { return }

	if let Ok( mut ui ) = q.single_mut()
	{	match now
		{	GameState::Pause => { ui.is_visible = false; let _ = state.pop(); }
			_                => { ui.is_visible = true ; let _ = state.push( GameState::Pause ); }
		}
		inkey.reset( KeyCode::Escape ); // https://bevy-cheatbook.github.io/programming/states.html#with-input
	}
}

//End of code.