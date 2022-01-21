//external modules
use bevy::{ prelude::*, diagnostic::*,};
// use bevy_prototype_lyon::{ prelude::*, entity::ShapeBundle };
// use bevy_egui::*;
// use rand::prelude::*;

//internal modules
mod types;
mod consts;
mod utils;

use types::*;
use consts::*;
use utils::*;

// mod ui;
// mod map;
// mod player;
// mod event;
// mod control_panel;
// use ui::*;
// use map::*;
// use player::*;
// use event::*;
// use control_panel::*;

//メイン関数
fn main()
{	let main_window = WindowDescriptor
	{	title    : APP_TITLE.to_string(),
		width    : SCREEN_WIDTH,
		height   : SCREEN_HEIGHT,
		resizable: false,
		..Default::default()
	};
	
	let mut app = App::new();
	app
	//----------------------------------------------------------------------------------------------
	.insert_resource( main_window )							// メインウィンドウ
	.insert_resource( ClearColor( SCREEN_BGCOLOR ) )		// 背景色
	.insert_resource( Msaa { samples: 4 } )					// アンチエイリアス
	//----------------------------------------------------------------------------------------------
	.add_plugins( DefaultPlugins )							// デフォルトプラグイン
	.add_plugin( FrameTimeDiagnosticsPlugin::default() )	// fps計測のプラグイン
	// .add_plugin( ShapePlugin )							// bevy_prototype_lyon
	// .add_plugin( EguiPlugin )							// bevy_egui
	//----------------------------------------------------------------------------------------------
	.add_state( GameState::Init )							// 状態遷移の初期値
	.init_resource::<SystemParameters>()					// 全体に影響する変数を格納するResource
	// .add_system_set										// ＜GameState::Init＞
	// (	SystemSet::on_enter( GameState::Init )			// ＜on_enter()＞
	// 		.with_system( start_preload_assets )			// Assetの事前ロード開始
	// )
	// .add_system_set										// ＜GameState::Init＞
	// (	SystemSet::on_update( GameState::Init )			// ＜on_update()＞
	// 		.with_system( change_state_after_loading )		// ロード完了⇒GameState::Startへ
	// )
	//----------------------------------------------------------------------------------------------
	.add_startup_system( spawn_camera )						// bevyのカメラ設置
	.add_system( handle_esc_key_for_pause )					// [Esc]でpause処理
	//----------------------------------------------------------------------------------------------
	// .add_plugin( PluginUi )
	// .add_plugin( PluginMap )
	// .add_plugin( PluginPlayer )
	// .add_plugin( PluginEvent )
	// .add_plugin( PluginControlPanel )
	//----------------------------------------------------------------------------------------------
	;

	#[cfg(not(target_arch = "wasm32"))]
	//----------------------------------------------------------------------------------------------
	app.add_system( toggle_window_mode );					// [Alt]+[Enter]でフルスクリーン
	//----------------------------------------------------------------------------------------------

	app.run();												// アプリの実行
}

////////////////////////////////////////////////////////////////////////////////////////////////////

// //Assetsのプリロードとハンドルの保存
// const PRELOAD_ASSET_FILES: [ &str; 3 ] =
// [	FONT_MESSAGE_TEXT,	//定義はui.rs
// 	FONT_TITLE_TEXT, 	//定義はui.rs
// 	WALL_SPRITE_FILE,	//定義はmap.rs
// ];
// struct LoadedAssets { preload: Vec<HandleUntyped> }

// //Assetの事前ロードを開始する
// fn start_preload_assets
// (	mut cmds: Commands,
// 	asset_svr: Res<AssetServer>,
// )
// {	//Assetのロードを開始
// 	let mut preload = Vec::new();
// 	PRELOAD_ASSET_FILES.iter()
// 		.for_each( | f | preload.push( asset_svr.load_untyped( *f ) ) );

// 	cmds.insert_resource( LoadedAssets { preload } );
// }

// //Assetのロードが完了したら、Stateを変更
// fn change_state_after_loading
// (	mut state : ResMut<State<GameState>>,
// 	assets: Res<LoadedAssets>,
// 	asset_svr: Res<AssetServer>,
// )
// {	for handle in assets.preload.iter()
// 	{	use bevy::asset::LoadState::*;
// 		match asset_svr.get_load_state( handle )
// 		{	Loaded => {}
// 			Failed => panic!(),	//ロードエラー⇒パニック
// 			_      => return,	//on_update()なので繰り返し関数が呼び出される
// 		}
// 	}

// 	//Startへ遷移する
// 	let _ = state.overwrite_set( GameState::Start );
// }

//End of code.