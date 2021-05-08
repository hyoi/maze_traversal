//external modules
use bevy::prelude::*;
use bevy_prototype_lyon::{ prelude::*, entity::ShapeBundle };
use bevy_egui::*;
use rand::prelude::*;

//internal modules
mod plugin_initialize;
mod plugin_gameplay;
mod map;
mod player;
mod sprite_bundles;

use plugin_initialize::*;
use plugin_gameplay::*;
use map::*;
use player::*;
use sprite_bundles::*;

////////////////////////////////////////////////////////////////////////////////

//アプリのTitle
const APP_TITLE: &str = "maze traversal";

//迷路の縦横のマス数
const MAP_WIDTH : usize = 30;
const MAP_HEIGHT: usize = 35;

//表示倍率、ウィンドウの縦横pixel数と背景色
const SCREEN_SCALING: usize = 3;
const PIXEL_PER_GRID: f32   = ( 8 * SCREEN_SCALING ) as f32;
const SCREEN_WIDTH  : f32   = PIXEL_PER_GRID * MAP_WIDTH  as f32;
const SCREEN_HEIGHT : f32   = PIXEL_PER_GRID * MAP_HEIGHT as f32;
const SCREEN_BGCOLOR: Color = Color::rgb_linear( 0.025, 0.025, 0.04 );

////////////////////////////////////////////////////////////////////////////////

//状態遷移
#[derive(Clone,Copy,Debug,Eq,PartialEq,Hash)]
pub enum GameState
{	Initialize,
	GameStart,
	GamePlay,
	GameClear,
	Pause,
}

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
	//--------------------------------------------------------------------------------
		.insert_resource( main_window )						// メインウィンドウ
		.insert_resource( ClearColor( SCREEN_BGCOLOR ) )	// 背景色
		.insert_resource( Msaa { samples: 4 } )				// アンチエイリアス
	//--------------------------------------------------------------------------------
		.add_plugins( DefaultPlugins )						// デフォルトプラグイン
		.add_plugin( ShapePlugin )							// bevy_prototype_lyonを使う
		.add_plugin( EguiPlugin )							// bevy_eguiを使う
	//--------------------------------------------------------------------------------
		.add_plugin( PluginInitialize )
		.add_plugin( PluginGamePlay )
	;
	//================================================================================
	#[cfg(target_arch = "wasm32")]
	app.add_plugin( bevy_webgl2::WebGL2Plugin );			// WASM用のプラグイン
	//================================================================================
	app.run();												// アプリの実行
}

//End of code.