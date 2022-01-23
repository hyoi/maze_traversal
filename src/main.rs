//external modules
use bevy::{ prelude::*, diagnostic::*,};

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
	//----------------------------------------------------------------------------------------------
	.add_state( GameState::Init )							// 状態遷移の初期値
	.init_resource::<SystemParameters>()					// 全体に影響する変数を格納するResource
	//----------------------------------------------------------------------------------------------
	.add_startup_system( spawn_camera )						// bevyのカメラ設置
	.add_system( handle_esc_key_for_pause )					// [Esc]でpause処理
	//----------------------------------------------------------------------------------------------
	.add_plugin( PluginFetchAssets )
	.add_plugin( PluginUi )
	.add_plugin( PluginMap )
	.add_plugin( PluginPlayer )
	//----------------------------------------------------------------------------------------------
	;

	#[cfg(not(target_arch = "wasm32"))]						// WASMで不要なキー操作
	app.add_system( toggle_window_mode );					// [Alt]+[Enter]でフルスクリーン

	app.run();												// アプリの実行
}

//End of code.