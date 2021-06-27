//external modules
use bevy::{ prelude::*, diagnostic::*,};
use bevy_prototype_lyon::{ prelude::*, entity::ShapeBundle };
//use bevy_egui::*;
use rand::prelude::*;

//internal modules
mod assets_and_ui;
mod map;
mod player;

use assets_and_ui::*;
use map::*;
use player::*;

////////////////////////////////////////////////////////////////////////////////////////////////////

//アプリのTitle
const APP_TITLE: &str = "maze traversal";

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
{	Init,
	Start,
	Play,
	Clear,
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
	.insert_resource( main_window )							// メインウィンドウ
	.insert_resource( ClearColor( SCREEN_BGCOLOR ) )		// 背景色
	.insert_resource( Msaa { samples: 4 } )					// アンチエイリアス
	//----------------------------------------------------------------------------------------------
	.add_plugins( DefaultPlugins )							// デフォルトプラグイン
	.add_plugin( FrameTimeDiagnosticsPlugin::default() )	// fps計測のプラグイン
	.add_plugin( ShapePlugin )								// bevy_prototype_lyon
//	.add_plugin( EguiPlugin )								// bevy_egui
	//----------------------------------------------------------------------------------------------
	.add_state( GameState::Init )							// 状態遷移のState初期値
	.init_resource::<GameRecord>()							// ゲームレコード
	//----------------------------------------------------------------------------------------------
	.add_plugin( PluginInit )
	.add_plugin( PluginMap )
	.add_plugin( PluginPlayer )
	//----------------------------------------------------------------------------------------------
	.add_startup_system( spawn_camera.system() )			// bevyのカメラ設置
	.add_system( handle_esc_key_for_pause.system() )		// [Esc]でpause処理
//	.add_system( egui_window.system() )						// ステージ数とスコアの表示
	//----------------------------------------------------------------------------------------------
	;

	#[cfg(not(target_arch = "wasm32"))]
	//----------------------------------------------------------------------------------------------
	app.add_system( toggle_window_mode.system() );			// [Alt]+[Enter]でフルスクリーン
	//----------------------------------------------------------------------------------------------

	#[cfg(target_arch = "wasm32")]
	//----------------------------------------------------------------------------------------------
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

////////////////////////////////////////////////////////////////////////////////////////////////////

//二次元配列の添え字から画面座標を算出する
pub fn conv_sprite_coordinates( x: i32, y: i32 ) -> ( f32, f32 )
{	let x = ( PIXEL_PER_GRID - SCREEN_WIDTH  ) / 2.0 + PIXEL_PER_GRID * x as f32;
	let y = ( SCREEN_HEIGHT - PIXEL_PER_GRID ) / 2.0 - PIXEL_PER_GRID * y as f32;
	( x, y )
}

////////////////////////////////////////////////////////////////////////////////////////////////////

// //ステージ数とスコアの表示
// fn egui_window( egui: Res<EguiContext>, stage: Res<GameStage>, record: Res<GameRecord> )
// {	egui::Window::new( APP_TITLE ).show
// 	(	egui.ctx(), |ui|
// 		{	ui.label( format!( "Stage: {}\nScore: {}", stage.level, record.score ) );
// 		}
// 	);
// }

//End of code.