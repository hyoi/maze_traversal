//external modules
use bevy::prelude::*;
use rand::prelude::*;

//internal modules
mod common;
use common::*;

mod fetch_assets;
mod ui;
mod map;
mod player;
mod chasers;

use fetch_assets::PluginFetchAssets;
use ui::PluginUi;
use map::PluginMap;
use player::PluginPlayer;
use chasers::PluginChaser;

//メイン関数
fn main()
{	let window = WindowDescriptor
	{	title    : APP_TITLE.to_string(),
		width    : SCREEN_WIDTH,
		height   : SCREEN_HEIGHT,
		resizable: false,
		..default()
	};
	let main_window = WindowPlugin { window, ..default() };

	let mut app = App::new();
	app
	//----------------------------------------------------------------------------------------------
	.insert_resource( ClearColor( SCREEN_BGCOLOR ) )		// 背景色
	.insert_resource( Msaa { samples: 4 } )					// アンチエイリアス
	.add_plugins( DefaultPlugins.set( main_window ) )		// デフォルトプラグイン、メインウィンドウ
	//----------------------------------------------------------------------------------------------
	.add_state( GameState::Init )							// 状態遷移の初期値
	//----------------------------------------------------------------------------------------------
	.add_startup_system( spawn_camera )						// bevyのカメラ設置
	//----------------------------------------------------------------------------------------------
	.add_plugin( PluginFetchAssets )
	.add_plugin( PluginUi )
	.add_plugin( PluginMap )
	.add_plugin( PluginPlayer )
	.add_plugin( PluginChaser )
	//----------------------------------------------------------------------------------------------
	;

	#[cfg(not(target_arch = "wasm32"))]						// WASMで不要なキー操作
	app.add_system( toggle_window_mode );					// [Alt]+[Enter]でフルスクリーン

	#[cfg(target_arch = "wasm32")]							//WASMで使用する
    app.add_plugin( bevy_web_resizer::Plugin );				//ブラウザ中央に表示する

	app.run();												// アプリの実行
}

//End of code.