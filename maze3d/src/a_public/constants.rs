use super::*;

pub const APP_TITLE : &str = ""; //アプリタイトル
pub const CARGO_NAME: &str = env!( "CARGO_PKG_NAME"    ); //cargo.ttomlの[package]name
pub const CARGO_VER : &str = env!( "CARGO_PKG_VERSION" ); //cargo.ttomlの[package]version

////////////////////////////////////////////////////////////////////////////////////////////////////

pub const DEBUG   : fn() -> bool = ||   cfg!( debug_assertions       ); //.run_if()用
pub const NOT_WASM: fn() -> bool = || ! cfg!( target_arch = "wasm32" ); //.run_if()用

////////////////////////////////////////////////////////////////////////////////////////////////////

pub const SCREEN_GRIDS_WIDTH : i32 = 43; //43 //ウィンドウ横幅(Grid)
pub const SCREEN_GRIDS_HEIGHT: i32 = 24; //24 //ウインドウ縦幅(Grid)

    const SCREEN_SCALING      : f32 = 4.0;
    const BASE_PIXELS_PER_GRID: i32 = 8;
pub const PIXELS_PER_GRID     : f32 = BASE_PIXELS_PER_GRID as f32 * SCREEN_SCALING; //1GridあたりのPixel数

pub const SCREEN_PIXELS_WIDTH : f32 = SCREEN_GRIDS_WIDTH  as f32 * PIXELS_PER_GRID; //ウィンドウ横幅(Pixel)
pub const SCREEN_PIXELS_HEIGHT: f32 = SCREEN_GRIDS_HEIGHT as f32 * PIXELS_PER_GRID; //ウィンドウ縦幅(Pixel)

////////////////////////////////////////////////////////////////////////////////////////////////////

pub static MAIN_WINDOW: Lazy<Option<Window>> = Lazy::new
(   ||
    {   let title = if APP_TITLE.is_empty() { CARGO_NAME } else { APP_TITLE };
        Some
        (   Window
            {   title     : format!( "{title} v{CARGO_VER}" ),
                resolution: ( SCREEN_PIXELS_WIDTH, SCREEN_PIXELS_HEIGHT ).into(),
                resizable : false,

                //WASM＆Android Chromeで表示不具合が発生する場合コメントアウトを検討する
                //fit_canvas_to_parent: true,

                ..default()
            }
        )
    }
);

////////////////////////////////////////////////////////////////////////////////////////////////////

pub const CAMERA2D_ORDER: isize = 1; //描画優先順
pub const CAMERA3D_ORDER: isize = 0; //描画優先順

use bevy::core_pipeline::clear_color::ClearColorConfig;
pub const CAMERA2D_BGCOLOR: ClearColorConfig = ClearColorConfig::None; //2Dは背景が透過
pub const CAMERA3D_BGCOLOR: ClearColorConfig = ClearColorConfig::Custom( Color::rgb( 0.13, 0.13, 0.18 ) );

pub const CAMERA3D_TRANSFORM: Transform = Transform::from_xyz( 0.0, 1.5, 5.0 ); //3Dカメラの初期位置

pub const LIGHT_BRIGHTNESS: f32 = 3000.0; //ライトの明るさ
pub const LIGHT_TRANSFORM : Transform = Transform::from_xyz( 4.0, 8.0, 4.0 ); //ライトの初期位置

////////////////////////////////////////////////////////////////////////////////////////////////////

use std::ops::Range;
pub const SCREEN_GRIDS_X_RANGE: Range<i32> = 0..SCREEN_GRIDS_WIDTH;  //ウィンドウ横幅(Grid)
pub const SCREEN_GRIDS_Y_RANGE: Range<i32> = 0..SCREEN_GRIDS_HEIGHT; //ウィンドウ縦幅(Grid)

////////////////////////////////////////////////////////////////////////////////////////////////////

pub const ASSETS_FONT_ORBITRON_BLACK      : &str = "fonts/Orbitron-Black.ttf";       //フォント
pub const ASSETS_FONT_REGGAEONE_REGULAR   : &str = "fonts/ReggaeOne-Regular.ttf";    //フォント
pub const ASSETS_FONT_PRESSSTART2P_REGULAR: &str = "fonts/PressStart2P-Regular.ttf"; //フォント

pub const ASSETS_SPRITE_DEBUG_GRID        : &str = "sprites/debug_grid.png";         //スプライト
pub const ASSETS_SPRITE_BRICK_WALL        : &str = "sprites/brick_wall.png";         //スプライト
pub const ASSETS_SPRITE_KANI_DOTOWN       : &str = "sprites/kani_DOTOWN.png";        //スプライト

//事前ロード対象
counted_array!
(   pub const FETCH_ASSETS: [ &str; _ ] =
    [   ASSETS_FONT_ORBITRON_BLACK,
        ASSETS_FONT_REGGAEONE_REGULAR,
        ASSETS_FONT_PRESSSTART2P_REGULAR,
        ASSETS_SPRITE_DEBUG_GRID,
        ASSETS_SPRITE_BRICK_WALL,
        ASSETS_SPRITE_KANI_DOTOWN,
    ]
);

////////////////////////////////////////////////////////////////////////////////////////////////////

pub const DEPTH_SPRITE_GAME_FRAME : f32 = 800.0; //スプライト重なり順

////////////////////////////////////////////////////////////////////////////////////////////////////

//画面デザイン(枠)
counted_array!
(   pub const DESIGN_SCREEN_FRAME: [ &str; _ ] =
    //   0123456789 123456789 123456789 123456789 123456789
    [   "0123456789012345678901234567890123456789012", //0----
        "1                                         #", //1
        "2                                         #", //2
        "3                                         #", //3
        "4                                         #", //4
        "5                                         5", //5
        "6                                         #", //6
        "7                                         #", //7
        "8                                         #", //8
        "9                                         #", //9
        "0                                         0", //10---
        "1                                         #", //11
        "2                                         #", //12
        "3                                         #", //13
        "4                                         #", //14
        "5                                         5", //15
        "6                                         #", //16
        "7                                         #", //17
        "8                                         #", //18
        "9                                         #", //19
        "0                                         0", //20---
        "1                                         #", //21
        "2                                         #", //22
        "3####5####0####5####0####5####0####5####0##", //23
    ]  //0123456789 123456789 123456789 123456789 123456789
);

////////////////////////////////////////////////////////////////////////////////////////////////////

//フッターに表示するtext UI
pub const NA3_2: &str = "###.##";
counted_array!
(   pub const FOOTER_LEFT_TEXT: [ MessageSect; _ ] =
    [   ( " FPS ", ASSETS_FONT_ORBITRON_BLACK      , PIXELS_PER_GRID * 0.6, Color::ORANGE ),
        ( NA3_2  , ASSETS_FONT_PRESSSTART2P_REGULAR, PIXELS_PER_GRID * 0.4, Color::WHITE  ),
    ]
);
counted_array!
(   pub const FOOTER_CENTER_TEXT: [ MessageSect; _ ] =
    [   ( "hyoi 2023", ASSETS_FONT_ORBITRON_BLACK, PIXELS_PER_GRID * 0.6, Color::ORANGE ),
    ]
);
counted_array!
(   pub const FOOTER_RIGHT_TEXT: [ MessageSect; _ ] =
    [   ( "Powered by Rust & Bevy ", ASSETS_FONT_ORBITRON_BLACK, PIXELS_PER_GRID * 0.6, Color::ORANGE ),
    ]
);

////////////////////////////////////////////////////////////////////////////////////////////////////

// //MAPの範囲の定数
// use std::ops::RangeInclusive;
// pub const RANGE_MAP_X      : RangeInclusive<i32> = 0..= MAP_GRIDS_WIDTH  - 1;	//MAP配列の添え字のレンジ
// pub const RANGE_MAP_Y      : RangeInclusive<i32> = 0..= MAP_GRIDS_HEIGHT - 1;	//MAP配列の添え字のレンジ
// pub const RANGE_MAP_INNER_X: RangeInclusive<i32> = 1..= MAP_GRIDS_WIDTH  - 2;	//掘削可能なレンジ（最外壁は掘れない）
// pub const RANGE_MAP_INNER_Y: RangeInclusive<i32> = 1..= MAP_GRIDS_HEIGHT - 2;	//掘削可能なレンジ（最外壁は掘れない）

// //MAP座標の上下左右を表す定数
// pub const UP   : DxDy = DxDy::Up;
// pub const LEFT : DxDy = DxDy::Left;
// pub const RIGHT: DxDy = DxDy::Right;
// pub const DOWN : DxDy = DxDy::Down;
// pub const FOUR_SIDES: [ DxDy; 4 ] = [ UP, LEFT, RIGHT, DOWN ];

// //MAPのマスの状態の制御に使うbit
// pub const BIT_HALL   : usize = 0b0001;
// pub const BIT_PASSAGE: usize = 0b0010;
// pub const BIT_DEADEND: usize = 0b0100;

// ////////////////////////////////////////////////////////////////////////////////////////////////////

// //Record
// pub const MAX_HP: f32 = 100.0;

//End of code.