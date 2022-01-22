//external modules
use bevy::{ prelude::*, diagnostic::*,};
// use bevy_egui::*;

//internal modules
mod types;
mod consts;
mod utils;

use types::*;
use consts::*;
use utils::*;

mod fetch_assets;
mod ui;
mod map;
mod player;

use fetch_assets::*;
use ui::*;
use map::*;
use player::*;

// mod event;
// mod control_panel;
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
	.add_plugin( PluginFetchAssets )
	.add_plugin( PluginUi )
	.add_plugin( PluginMap )
	.add_plugin( PluginPlayer )
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

//End of code.