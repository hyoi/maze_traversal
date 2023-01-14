use super::*;

//アプリ名とバージョン
pub const APP_TITLE : &str = "MazeTraversal";             //アプリ名
pub const _CARGO_VER: &str = env!( "CARGO_PKG_VERSION" ); //cargo.tomlの[package]version

//マップの縦横マス数
pub const MAP_GRIDS_SHARP_PLANE: i32 = 20; //shape::Plane { size: XXXX as f32 }
pub const MAP_GRIDS_WIDTH      : i32 = MAP_GRIDS_SHARP_PLANE; //マップ横幅
pub const MAP_GRIDS_HEIGHT     : i32 = MAP_GRIDS_SHARP_PLANE; //マップ縦幅

//ウィンドウ画面のマス数
pub const SCREEN_GRIDS_WIDTH : i32 = 43; //21,27,33,43 //ウィンドウ横幅
pub const SCREEN_GRIDS_HEIGHT: i32 = 24; //16,20,25,32 //ウインドウ縦幅

//ウィンドウ画面のサイズと背景色
const BASE_PIXELS_PER_GRID: i32 = 8;   //8pixel × 8pixelのマス
const SCREEN_SCALING      : f32 = 4.0; //7.0倍に拡大
pub const PIXELS_PER_GRID : f32 = BASE_PIXELS_PER_GRID as f32 * SCREEN_SCALING; //1GridあたりのPixel数

pub const SCREEN_PIXELS_WIDTH : f32 = SCREEN_GRIDS_WIDTH  as f32 * PIXELS_PER_GRID; //ウィンドウ横幅
pub const SCREEN_PIXELS_HEIGHT: f32 = SCREEN_GRIDS_HEIGHT as f32 * PIXELS_PER_GRID; //ウィンドウ縦幅

pub const SCREEN_BACKGROUND_COLOR: Color = Color::rgb( 0.13, 0.13, 0.18 ); //ウィンドウ背景色

use std::ops::Range;
pub const SCREEN_GRIDS_RANGE_X: Range<i32> = 0..SCREEN_GRIDS_WIDTH;  //ウィンドウ横幅
pub const SCREEN_GRIDS_RANGE_Y: Range<i32> = 0..SCREEN_GRIDS_HEIGHT; //ウィンドウ縦幅

//画面デザイン(枠)
pub const DESIGN_GAME_FRAME: [ &str; SCREEN_GRIDS_HEIGHT as usize ] =
//   0123456789 123456789 123456789 123456789 12
[   "###########################################", //0----
    "#                               ###########", //1
    "#                               ###########", //2
    "#                               ###########", //3
    "#                               ###########", //4
    "#                               ###########", //5
    "#                               ###########", //6
    "#                               ###########", //7
    "#                               ###########", //8
    "#                               ###########", //9
    "#                               ###########", //10---
    "#                               ###########", //11
    "#                               ###########", //12
    "#                               #=========#", //13
    "#                               #=========#", //14
    "#                               #=========#", //15
    "#                               #=========#", //16
    "#                               #=========#", //17
    "#                               #=========#", //18
    "#                               #=========#", //19
    "#                               #=========#", //20---
    "#                               #=========#", //21
    "###########################################", //22
    "===========================================", //23
]; //0123456789 123456789 123456789 1234

////////////////////////////////////////////////////////////////////////////////////////////////////

pub const ASSETS_FONT_ORBITRON_BLACK      : &str = "fonts/Orbitron-Black.ttf";       //フォント
pub const ASSETS_FONT_REGGAEONE_REGULAR   : &str = "fonts/ReggaeOne-Regular.ttf";    //フォント
pub const ASSETS_FONT_PRESSSTART2P_REGULAR: &str = "fonts/PressStart2P-Regular.ttf"; //フォント
pub const ASSETS_SPRITE_KANI_DOTOWN       : &str = "sprites/kani_DOTOWN.png";        //スプライト
pub const ASSETS_SPRITE_BRICK_WALL        : &str = "sprites/brick_wall.png";         //スプライト

//事前ロード対象のAsset
pub const FETCH_ASSETS: [ &str; 5 ] =
[   ASSETS_FONT_ORBITRON_BLACK,
    ASSETS_FONT_REGGAEONE_REGULAR,
    ASSETS_FONT_PRESSSTART2P_REGULAR,
    ASSETS_SPRITE_KANI_DOTOWN,
    ASSETS_SPRITE_BRICK_WALL,
];

pub const  DEPTH_SPRITE_GAME_FRAME : f32 = 100.0; //スプライト重なり順

// //Assets（フォント、画像...etc）
// pub const FONT_ORBITRON_BLACK	: &str = "fonts/Orbitron-Black.ttf";
// pub const FONT_REGGAEONE_REGULAR: &str = "fonts/ReggaeOne-Regular.ttf";
// pub const IMAGE_SPRITE_WALL		: &str = "sprites/wall.png";
// pub const IMAGE_SPRITE_COIN		: &str = "sprites/coin.png";
// pub const IMAGE_SPRITE_KANI		: &str = "sprites/kani_DOTOWN.png";

// //事前ロード対象のAsset
// pub const FETCH_ASSETS: [ &str; 5 ] =
// [	FONT_ORBITRON_BLACK,
//     FONT_REGGAEONE_REGULAR,
//     IMAGE_SPRITE_WALL,
//     IMAGE_SPRITE_COIN,
//     IMAGE_SPRITE_KANI,
// ];

////////////////////////////////////////////////////////////////////////////////////////////////////

//ゲームパッドのID
pub const GAMEPAD: Gamepad = Gamepad { id: 0 }; //Todo: pad 0番決め打ちでいいいのか？

//FULLSCREENのキーとパッドボタン
pub const _KEY_ALT_RIGHT: KeyCode = KeyCode::RAlt;
pub const _KEY_ALT_LEFT : KeyCode = KeyCode::LAlt;
pub const _KEY_FULLSCREEN: KeyCode = KeyCode::Return;
pub const _BUTTON_FULLSCREEN: GamepadButtonType = GamepadButtonType::West; //PS4の□ボタン

////////////////////////////////////////////////////////////////////////////////////////////////////

//MAPの範囲の定数
use std::ops::RangeInclusive;
pub const RANGE_MAP_X      : RangeInclusive<i32> = 0..= MAP_GRIDS_WIDTH  - 1;	//MAP配列の添え字のレンジ
pub const RANGE_MAP_Y      : RangeInclusive<i32> = 0..= MAP_GRIDS_HEIGHT - 1;	//MAP配列の添え字のレンジ
pub const RANGE_MAP_INNER_X: RangeInclusive<i32> = 1..= MAP_GRIDS_WIDTH  - 2;	//掘削可能なレンジ（最外壁は掘れない）
pub const RANGE_MAP_INNER_Y: RangeInclusive<i32> = 1..= MAP_GRIDS_HEIGHT - 2;	//掘削可能なレンジ（最外壁は掘れない）

//MAP座標の上下左右を表す定数
pub const UP   : DxDy = DxDy::Up;
pub const LEFT : DxDy = DxDy::Left;
pub const RIGHT: DxDy = DxDy::Right;
pub const DOWN : DxDy = DxDy::Down;
pub const FOUR_SIDES: [ DxDy; 4 ] = [ UP, LEFT, RIGHT, DOWN ];

//MAPのマスの状態の制御に使うbit
pub const BIT_HALL   : usize = 0b0001;
pub const BIT_PASSAGE: usize = 0b0010;
pub const BIT_DEADEND: usize = 0b0100;

////////////////////////////////////////////////////////////////////////////////////////////////////

//Spriteの深さ
// pub const SPRITE_DEPTH_CHASER: f32 = 30.0;	//追手
// pub const SPRITE_DEPTH_PLAYER: f32 = 20.0;	//自機
// pub const SPRITE_DEPTH_MAZE  : f32 = 10.0;	//壁、コイン etc
// pub const SPRITE_DEPTH_DEBUG : f32 =  5.0;	//広間

////////////////////////////////////////////////////////////////////////////////////////////////////

//Record
pub const MAX_HP: f32 = 100.0;

//End of code.