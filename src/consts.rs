use super::*;

//マップの縦横のマス数
pub const MAP_WIDTH : usize = 35;	//66
pub const MAP_HEIGHT: usize = 35;

//アプリのTitle
pub const APP_TITLE: &str = "maze traversal";

//表示倍率、ウィンドウの縦横pixel数と背景色
pub const SCREEN_SCALING: usize = 3;
pub const PIXEL_PER_GRID: f32   = ( 8 * SCREEN_SCALING ) as f32;
pub const SCREEN_WIDTH  : f32   = PIXEL_PER_GRID * MAP_WIDTH  as f32;
pub const SCREEN_HEIGHT : f32   = PIXEL_PER_GRID * ( MAP_HEIGHT as f32 + 2.0 );
pub const SCREEN_BGCOLOR: Color = Color::rgb_linear( 0.025, 0.025, 0.04 );

////////////////////////////////////////////////////////////////////////////////////////////////////

//事前ロード対象のAsset（フォント、画像...etc）
pub const FONT_MESSAGE_TEXT: &str = "fonts/Orbitron-Black.ttf";
pub const FONT_TITLE_TEXT  : &str = "fonts/ReggaeOne-Regular.ttf";
pub const WALL_SPRITE_FILE : &str = "sprites/wall.png";
pub const COIN_SPRITE_FILE : &str = "sprites/coin.png";

pub const FETCH_ASSETS: [ &str; 4 ] =
[	FONT_MESSAGE_TEXT,
	FONT_TITLE_TEXT,
	WALL_SPRITE_FILE,
	COIN_SPRITE_FILE,
];

//TEXT UIのメッセージセクションの型
pub type MessageSect<'a> = ( &'a str, &'a str, f32, Color );

#[derive(Component)]
pub struct MessagePause;
pub const MESSAGE_PAUSE: [ MessageSect; 1 ] =
[	( "P A U S E", FONT_MESSAGE_TEXT, PIXEL_PER_GRID * 5.0, Color::SILVER ),
];

#[derive(Component)]
pub struct MessageClear;
pub const MESSAGE_CLEAR: [ MessageSect; 3 ] =
[	( "C L E A R !!\n"   , FONT_MESSAGE_TEXT, PIXEL_PER_GRID * 5.0, Color::GOLD  ),
	( "Next stage...\n\n", FONT_MESSAGE_TEXT, PIXEL_PER_GRID * 2.0, Color::WHITE ),
	( ""                 , FONT_MESSAGE_TEXT, PIXEL_PER_GRID * 4.0, Color::WHITE ),
];

#[derive(Component)]
pub struct MessageEvent;
pub const MESSAGE_EVENT: [ MessageSect; 3 ] =
[	( "E V E N T !!\n", FONT_TITLE_TEXT, PIXEL_PER_GRID * 5.0, Color::GOLD  ),
	( "戦闘中...\n\n"  , FONT_TITLE_TEXT, PIXEL_PER_GRID * 2.0, Color::WHITE ),
	( "Hit SPACE Kry!", FONT_TITLE_TEXT, PIXEL_PER_GRID * 2.5, Color::GOLD ),
];

pub const NA_STR3: &str = "---";
pub const NA_STR5: &str = "-----";

#[derive(Component)]
pub struct UiUpperLeft;
pub const UI_UPPER_LEFT: [ MessageSect; 2 ] =
[	( " HP " , FONT_MESSAGE_TEXT, PIXEL_PER_GRID * 0.8, Color::ORANGE ),
	( NA_STR3, FONT_MESSAGE_TEXT, PIXEL_PER_GRID * 1.0, Color::WHITE  ),
];
/*
#[derive(Component)]
pub struct UiUpperCenter;
pub const UI_UPPER_CENTER: [ MessageSect; 2 ] =
[	( APP_TITLE, FONT_TITLE_TEXT, PIXEL_PER_GRID * 1.3, Color::ORANGE ),
	( "迷路踏破", FONT_TITLE_TEXT, PIXEL_PER_GRID * 1.6, Color::WHITE  ),
];
*/
#[derive(Component)]
pub struct UiUpperRight;
pub const UI_UPPER_RIGHT: [ MessageSect; 4 ] =
[	( ""       , FONT_MESSAGE_TEXT, PIXEL_PER_GRID * 1.0, Color::WHITE  ),
	( "GOLD / ", FONT_MESSAGE_TEXT, PIXEL_PER_GRID * 0.8, Color::ORANGE ),
	( ""       , FONT_MESSAGE_TEXT, PIXEL_PER_GRID * 1.0, Color::WHITE  ),
	( "FLOOR " , FONT_MESSAGE_TEXT, PIXEL_PER_GRID * 0.8, Color::ORANGE ),
];

#[derive(Component)]
pub struct UiLowerLeft;
pub const UI_LOWER_LEFT: [ MessageSect; 2 ] =
[	( "FPS " , FONT_MESSAGE_TEXT, PIXEL_PER_GRID * 0.8, Color::ORANGE ),
	( NA_STR3, FONT_MESSAGE_TEXT, PIXEL_PER_GRID * 1.0, Color::WHITE  ),
];

#[derive(Component)]
pub struct UiLowerCenter;
pub const UI_LOWER_CENTER: [ MessageSect; 1 ] =
[	( "2021 - 2022 hyoi", FONT_MESSAGE_TEXT, PIXEL_PER_GRID * 0.7, Color::WHITE ),
];

#[derive(Component)]
pub struct UiLowerRight;
pub const UI_LOWER_RIGHT: [ MessageSect; 1 ] =
[	( "powered by Rust&Bevy", FONT_MESSAGE_TEXT, PIXEL_PER_GRID * 0.7, Color::WHITE ),
];

////////////////////////////////////////////////////////////////////////////////////////////////////

//LIFE GAUGE
#[derive(Component)]
pub struct HpGauge;
pub const GAUGE_RECTANGLE: ( f32, f32, f32, f32 ) = 
(	PIXEL_PER_GRID *  8.9 - SCREEN_WIDTH  / 2.0,	//X軸：画面中央からやや左より
	PIXEL_PER_GRID * -0.7 + SCREEN_HEIGHT / 2.0,	//Y軸：画面上端からやや下がった位置
	PIXEL_PER_GRID * 15.0,							//幅
	PIXEL_PER_GRID *  0.2,							//高さ
);
pub const GAUGE_DEPTH: f32 = 30.0;

//End of code.