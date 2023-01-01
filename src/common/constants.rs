use super::*;

//アプリのTitle
pub const APP_TITLE: &str = "Maze Traversal";

//マップの縦横のマス数
pub const MAP_WIDTH : i32 = 20;	//66
pub const MAP_HEIGHT: i32 = MAP_WIDTH;	//35

pub const MAP_WH_SIZE: f32 = MAP_WIDTH as f32;

//画面の縦横のマス数
pub const GRID_WIDTH : i32 = MAP_WIDTH;
pub const GRID_HEIGHT: i32 = MAP_HEIGHT + 2;	//マップの高さ＋ヘッダ＋フッタ

//表示倍率、ウィンドウの縦横pixel数と背景色
pub const SCREEN_SCALING: usize = 7;
pub const PIXEL_PER_GRID: f32   = ( 8 * SCREEN_SCALING ) as f32;
pub const SCREEN_WIDTH  : f32   = PIXEL_PER_GRID * GRID_WIDTH  as f32;
pub const SCREEN_HEIGHT : f32   = PIXEL_PER_GRID * GRID_HEIGHT as f32;
pub const SCREEN_BGCOLOR: Color = Color::rgb( 0.1, 0.1, 0.1 );

////////////////////////////////////////////////////////////////////////////////////////////////////

//Assets（フォント、画像...etc）
pub const FONT_ORBITRON_BLACK	: &str = "fonts/Orbitron-Black.ttf";
pub const FONT_REGGAEONE_REGULAR: &str = "fonts/ReggaeOne-Regular.ttf";
pub const IMAGE_SPRITE_WALL		: &str = "sprites/wall.png";
pub const IMAGE_SPRITE_COIN		: &str = "sprites/coin.png";
pub const IMAGE_SPRITE_KANI		: &str = "sprites/kani_DOTOWN.png";

//事前ロード対象のAsset
pub const FETCH_ASSETS: [ &str; 5 ] =
[	FONT_ORBITRON_BLACK,
	FONT_REGGAEONE_REGULAR,
	IMAGE_SPRITE_WALL,
	IMAGE_SPRITE_COIN,
	IMAGE_SPRITE_KANI,
];

////////////////////////////////////////////////////////////////////////////////////////////////////

//MAPの範囲の定数
use std::ops::RangeInclusive;
pub const RANGE_MAP_X      : RangeInclusive<i32> = 0..= MAP_WIDTH  - 1;	//MAP配列の添え字のレンジ
pub const RANGE_MAP_Y      : RangeInclusive<i32> = 0..= MAP_HEIGHT - 1;	//MAP配列の添え字のレンジ
pub const RANGE_MAP_INNER_X: RangeInclusive<i32> = 1..= MAP_WIDTH  - 2;	//掘削可能なレンジ（最外壁は掘れない）
pub const RANGE_MAP_INNER_Y: RangeInclusive<i32> = 1..= MAP_HEIGHT - 2;	//掘削可能なレンジ（最外壁は掘れない）

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
pub const SPRITE_DEPTH_CHASER: f32 = 30.0;	//追手
pub const SPRITE_DEPTH_PLAYER: f32 = 20.0;	//自機
pub const SPRITE_DEPTH_MAZE  : f32 = 10.0;	//壁、コイン etc
pub const SPRITE_DEPTH_DEBUG : f32 =  5.0;	//広間

////////////////////////////////////////////////////////////////////////////////////////////////////

//Record
pub const MAX_HP: f32 = 100.0;

//End of code.