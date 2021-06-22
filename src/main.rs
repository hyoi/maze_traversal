//external modules
use bevy::prelude::*;
use bevy_prototype_lyon::{ prelude::*, entity::ShapeBundle };
//use bevy_egui::*;
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

////////////////////////////////////////////////////////////////////////////////////////////////////

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

////////////////////////////////////////////////////////////////////////////////////////////////////

//状態遷移
#[derive(Clone,Copy,Debug,Eq,PartialEq,Hash)]
pub enum GameState
{	Initialize,
	GameStart,
	GamePlay,
	GameClear,
	Pause,
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
	.insert_resource( main_window )						// メインウィンドウ
	.insert_resource( ClearColor( SCREEN_BGCOLOR ) )	// 背景色
	.insert_resource( Msaa { samples: 4 } )				// アンチエイリアス
	//----------------------------------------------------------------------------------------------
	.add_plugins( DefaultPlugins )						// デフォルトプラグイン
	.add_plugin( ShapePlugin )							// bevy_prototype_lyonを使う
//	.add_plugin( EguiPlugin )							// bevy_eguiを使う
	//----------------------------------------------------------------------------------------------
	.add_state( GameState::Initialize )					// 状態遷移のState初期値
	.add_event::<GameState>()							// 状態遷移のEventキュー
	.init_resource::<GameRecord>()						// ゲームレコード
	.init_resource::<GameStage>()						// ゲームステージ
	//----------------------------------------------------------------------------------------------
	.add_plugin( PluginInitialize )
	.add_plugin( PluginGamePlay )
	//----------------------------------------------------------------------------------------------
	.add_startup_system( spawn_camera.system() )		// bevyのカメラ設置
	.add_system( judge_pause_key_input.system() )		// pause処理
//	.add_system( egui_window.system() )					// ステージ数とスコアの表示
	//----------------------------------------------------------------------------------------------
	;

	//----------------------------------------------------------------------------------------------
	#[cfg(not(target_arch = "wasm32"))]
	app.add_system( toggle_window_mode.system() );			// [Alt]+[Enter]でフルスクリーン
	//----------------------------------------------------------------------------------------------

	//----------------------------------------------------------------------------------------------
	#[cfg(target_arch = "wasm32")]
	app.add_plugin( bevy_webgl2::WebGL2Plugin );			// WASM用のプラグイン
	//----------------------------------------------------------------------------------------------

	app.run();												// アプリの実行
}

////////////////////////////////////////////////////////////////////////////////////////////////////

//定義と定数

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

////////////////////////////////////////////////////////////////////////////////////////////////////

//bevyのカメラの設置
fn spawn_camera( mut cmds: Commands )
{	cmds.spawn_bundle( UiCameraBundle::default() );
	cmds.spawn_bundle( OrthographicCameraBundle::new_2d() );
}

////////////////////////////////////////////////////////////////////////////////////////////////////

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

////////////////////////////////////////////////////////////////////////////////////////////////////

// //ステージ数とスコアの表示
// fn egui_window( egui: Res<EguiContext>, stage: Res<GameStage>, record: Res<GameRecord> )
// {	egui::Window::new( APP_TITLE ).show
// 	(	egui.ctx(),
// 		|ui| { ui.label( format!( "Stage: {}\nScore: {}", stage.level, record.score ) ); }
// 	);
// }

//End of code.