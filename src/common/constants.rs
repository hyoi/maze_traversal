use super::*;

//アプリのTitle
pub const APP_TITLE: &str = "maze traversal";

//マップの縦横のマス数
pub const MAP_WIDTH : usize = 35;	//66
pub const MAP_HEIGHT: usize = 35;

//画面の縦横のマス数
pub const GRID_WIDTH : usize = MAP_WIDTH;
pub const GRID_HEIGHT: usize = MAP_HEIGHT + 2;	//マップの高さ＋ヘッダ＋フッタ

//表示倍率、ウィンドウの縦横pixel数と背景色
pub const SCREEN_SCALING: usize = 3;
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

//事前ロード対象のAsset
pub const FETCH_ASSETS: [ &str; 4 ] =
[	FONT_ORBITRON_BLACK,
	FONT_REGGAEONE_REGULAR,
	IMAGE_SPRITE_WALL,
	IMAGE_SPRITE_COIN,
];

////////////////////////////////////////////////////////////////////////////////////////////////////

//TEXT UIのメッセージセクションの型
pub type MessageSect<'a> = ( &'a str, &'a str, f32, Color );

#[derive(Component)]
pub struct MessagePause;
pub const MESSAGE_PAUSE: [ MessageSect; 1 ] =
[	( "P A U S E", FONT_ORBITRON_BLACK, PIXEL_PER_GRID * 5.0, Color::SILVER ),
];

#[derive(Component,Debug)]
pub struct MessageClear;
pub const MESSAGE_CLEAR: [ MessageSect; 3 ] =
[	( "C L E A R !!\n"   , FONT_ORBITRON_BLACK, PIXEL_PER_GRID * 5.0, Color::GOLD  ),
	( "Next floor...\n\n", FONT_ORBITRON_BLACK, PIXEL_PER_GRID * 2.0, Color::WHITE ),
	( ""                 , FONT_ORBITRON_BLACK, PIXEL_PER_GRID * 4.0, Color::WHITE ),
];

#[derive(Component,Debug)]
pub struct MessageOver;
pub const MESSAGE_OVER: [ MessageSect; 3 ] =
[	( "GAME OVER\n", FONT_REGGAEONE_REGULAR, PIXEL_PER_GRID * 5.0, Color::RED ),
	( ""           , FONT_REGGAEONE_REGULAR, PIXEL_PER_GRID * 4.0, Color::RED ),
	( ""           , FONT_REGGAEONE_REGULAR, PIXEL_PER_GRID * 4.0, Color::RED ),
];

#[derive(Component)]
pub struct MessageEvent;
pub const MESSAGE_EVENT: [ MessageSect; 3 ] =
[	( "E V E N T !!\n", FONT_REGGAEONE_REGULAR, PIXEL_PER_GRID * 5.0, Color::GOLD  ),
	( "戦闘中...\n\n"  , FONT_REGGAEONE_REGULAR, PIXEL_PER_GRID * 2.0, Color::WHITE ),
	( "Hit SPACE Kry!", FONT_REGGAEONE_REGULAR, PIXEL_PER_GRID * 2.5, Color::GOLD ),
];

#[derive(Component)]
pub struct UiUpperLeft;
pub const UI_UPPER_LEFT: [ MessageSect; 2 ] =
[	( " HP ", FONT_ORBITRON_BLACK, PIXEL_PER_GRID * 0.8, Color::ORANGE ),
	( ""    , FONT_ORBITRON_BLACK, PIXEL_PER_GRID * 1.0, Color::WHITE  ),
];
/*
#[derive(Component)]
pub struct UiUpperCenter;
pub const UI_UPPER_CENTER: [ MessageSect; 2 ] =
[	( APP_TITLE, FONT_REGGAEONE_REGULAR, PIXEL_PER_GRID * 1.3, Color::ORANGE ),
	( "迷路踏破", FONT_REGGAEONE_REGULAR, PIXEL_PER_GRID * 1.6, Color::WHITE  ),
];
*/
#[derive(Component)]
pub struct UiUpperRight;
pub const UI_UPPER_RIGHT: [ MessageSect; 4 ] =
[	( ""        , FONT_ORBITRON_BLACK, PIXEL_PER_GRID * 1.0, Color::WHITE  ),
	( " GOLD / ", FONT_ORBITRON_BLACK, PIXEL_PER_GRID * 0.8, Color::ORANGE ),
	( ""        , FONT_ORBITRON_BLACK, PIXEL_PER_GRID * 1.0, Color::WHITE  ),
	( " FLOOR " , FONT_ORBITRON_BLACK, PIXEL_PER_GRID * 0.8, Color::ORANGE ),
];

#[derive(Component)]
pub struct UiLowerLeft;
pub const UI_LOWER_LEFT: [ MessageSect; 2 ] =
[	( "FPS ", FONT_ORBITRON_BLACK, PIXEL_PER_GRID * 0.8, Color::ORANGE ),
	( ""    , FONT_ORBITRON_BLACK, PIXEL_PER_GRID * 1.0, Color::WHITE  ),
];

#[derive(Component)]
pub struct UiLowerCenter;
pub const UI_LOWER_CENTER: [ MessageSect; 1 ] =
[	( "2021 - 2022 hyoi", FONT_ORBITRON_BLACK, PIXEL_PER_GRID * 0.7, Color::WHITE ),
];

#[derive(Component)]
pub struct UiLowerRight;
pub const UI_LOWER_RIGHT: [ MessageSect; 1 ] =
[	( "powered by Rust&Bevy", FONT_ORBITRON_BLACK, PIXEL_PER_GRID * 0.7, Color::WHITE ),
];

////////////////////////////////////////////////////////////////////////////////////////////////////

//MAPの範囲の定数
use std::ops::RangeInclusive;
pub const RANGE_MAP_X      : RangeInclusive<usize> = 0..= MAP_WIDTH  - 1;	//MAP配列の添え字のレンジ
pub const RANGE_MAP_Y      : RangeInclusive<usize> = 0..= MAP_HEIGHT - 1;	//MAP配列の添え字のレンジ
pub const RANGE_MAP_INNER_X: RangeInclusive<usize> = 1..= MAP_WIDTH  - 2;	//掘削可能なレンジ（最外壁は掘れない）
pub const RANGE_MAP_INNER_Y: RangeInclusive<usize> = 1..= MAP_HEIGHT - 2;	//掘削可能なレンジ（最外壁は掘れない）

//MAP座標の上下左右を表す定数
pub const UP   : DxDy = DxDy { dx:  0, dy: -1 };
pub const LEFT : DxDy = DxDy { dx: -1, dy:  0 };
pub const RIGHT: DxDy = DxDy { dx:  1, dy:  0 };
pub const DOWN : DxDy = DxDy { dx:  0, dy:  1 };
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
// pub const SPRITE_DEPTH_DEBUG : f32 =  5.0;	//広間

////////////////////////////////////////////////////////////////////////////////////////////////////

//Record
pub const MAX_HP: f32 = 100.0;

//End of code.