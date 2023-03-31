use super::*;

//ゲームの状態
#[allow( dead_code )]
#[derive( Clone, Copy, Eq, PartialEq, Hash, Debug, Default, States )]
#[derive( MyConstState )]
pub enum MyState
{   #[default] InitApp,
    TitleDemo, DemoLoop,
    GameStart, StageStart, MainLoop, StageClear, GameOver,
    Pause, Debug,
}

////////////////////////////////////////////////////////////////////////////////////////////////////

//bevyがre-exportしているglamの型に別名を付ける
pub type Px2d =  Vec2;
pub type Px3d =  Vec3;
pub type Grid = IVec2;

pub trait MyGridTrait
{   fn to_screen_pixel( &self ) -> Px2d;
}
impl MyGridTrait for Grid
{   //Gridからスクリーンの座標(Px2d)を算出する
    fn to_screen_pixel( &self ) -> Px2d
    {   let x = ( PIXELS_PER_GRID - SCREEN_PIXELS_WIDTH  ) / 2.0 + PIXELS_PER_GRID * self.x as f32;
        let y = ( SCREEN_PIXELS_HEIGHT - PIXELS_PER_GRID ) / 2.0 - PIXELS_PER_GRID * self.y as f32;
        Px2d::new( x, y )
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

//Camera3dのComponent
#[derive( Component )] pub struct MovingCamera;

//text UIのメッセージセクションの型
pub type MessageSect<'a> =
(   &'a str, //表示文字列
    &'a str, //フォントのファイル名
    f32,     //フォンtのピクセル数（PIXELS_PER_GRIDＸ0.7 等）
    Color,   //文字の色（Bevy::Color）
);

//text UIのComponent
#[derive( Component )] pub struct FooterLeft;
#[derive( Component )] pub struct FooterCenter;
#[derive( Component )] pub struct FooterRight;

////////////////////////////////////////////////////////////////////////////////////////////////////

// //submodules
// mod grid_pixel_dxdy;

// //re-export
// pub use grid_pixel_dxdy::*;

// ////////////////////////////////////////////////////////////////////////////////////////////////////

// //スコア等のResource
// #[derive(Resource)]
// pub struct Record
// {	pub stage: usize,
// 	pub score: usize,
// 	pub hp	 : f32,
// }
// impl Default for Record
// {	fn default() -> Self
// 	{	Self
// 		{	stage: 0,
// 			score: 0,
// 			hp   : MAX_HP,
// 		}
// 	}
// }



// ////////////////////////////////////////////////////////////////////////////////////////////////////

// //MAPのマスの種類
// #[derive(Copy,Clone,PartialEq,Eq)]
// pub enum MapObj
// {	Wall,
// 	Passage, //通常の道
// 	DeadEnd, //迷路作成関数のアルゴリズム用（袋小路の目印）
// 	Coin ( Option<Entity>, usize ),	//スプライトのEntity IDとゴールドの数値
// 	Goal ( Option<Entity> ),		//スプライトのEntity ID
// }

// //MAP情報のResource
// #[derive(Resource)]
// pub struct GameMap
// {	rng: rand::prelude::StdRng,	//再現性がある乱数を使いたいので
// 	map  : [ [ MapObj; MAP_GRIDS_HEIGHT as usize ]; MAP_GRIDS_WIDTH as usize ],
// 	bits : [ [ usize ; MAP_GRIDS_HEIGHT as usize ]; MAP_GRIDS_WIDTH as usize ],
// 	start: MapGrid,
// 	goal : MapGrid,
// 	halls: usize,	//広間のマス数
// }
// impl Default for GameMap
// {	fn default() -> Self
// 	{	//開発で迷路作成に再現性が必要な場合、乱数シードを固定する。本番はランダムにする。
// 		let seed = if cfg!( debug_assertions ) { 1234567890 } else { rand::thread_rng().gen::<u64>() };

// 		Self
// 		{	rng  : StdRng::seed_from_u64( seed ),	
// 			map  : [ [ MapObj::Wall; MAP_GRIDS_HEIGHT as usize ]; MAP_GRIDS_WIDTH as usize ],
// 			bits : [ [ 0           ; MAP_GRIDS_HEIGHT as usize ]; MAP_GRIDS_WIDTH as usize ],
// 			start: MapGrid::default(),
// 			goal : MapGrid::default(),
// 			halls: 0,
// 		}
// 	}
// }

// impl GameMap
// {	//GameMap構造体のアクセサ
// 	pub fn rng( &mut self ) -> &mut rand::prelude::StdRng { &mut self.rng }

// 	pub fn mapobj( &self, grid: MapGrid ) -> MapObj { self.map [ *grid.x() as usize ][ *grid.y() as usize ] }
// 	pub fn mapobj_mut( &mut self, grid: MapGrid ) -> &mut MapObj { &mut self.map[ *grid.x() as usize ][ *grid.y() as usize ] }

// 	pub fn bits( &self, grid: MapGrid ) -> usize { self.bits[ *grid.x() as usize ][ *grid.y() as usize ] }
// 	fn bits_mut( &mut self, grid: MapGrid ) -> &mut usize { &mut self.bits[ *grid.x() as usize ][ *grid.y() as usize ] }

// 	pub fn start( &self ) -> MapGrid { self.start }
// 	pub fn start_mut( &mut self ) -> &mut MapGrid { &mut self.start }

// //	pub fn goal( &self ) -> MapGrid { self.goal }
// 	pub fn goal_mut( &mut self ) -> &mut MapGrid { &mut self.goal }

// //	pub fn halls( &self ) -> usize { self.halls }
// 	pub fn halls_mut( &mut self ) -> &mut usize { &mut self.halls }

// 	//指定されたマスのフラグ操作
// //	pub fn is_hall   ( &self, grid: MapGrid ) -> bool { self.bits( grid ) & BIT_HALL    != 0 }
// //	pub fn is_passage( &self, grid: MapGrid ) -> bool { self.bits( grid ) & BIT_PASSAGE != 0 }
// 	pub fn is_deadend( &self, grid: MapGrid ) -> bool { self.bits( grid ) & BIT_DEADEND != 0 }
// 	pub fn set_flag_hall   ( &mut self, grid: MapGrid ) { *self.bits_mut( grid ) |= BIT_HALL    }
// 	pub fn set_flag_passage( &mut self, grid: MapGrid ) { *self.bits_mut( grid ) |= BIT_PASSAGE }
// 	pub fn set_flag_deadend( &mut self, grid: MapGrid ) { *self.bits_mut( grid ) |= BIT_DEADEND }

// 	//配列を初期化する
// 	pub fn clear_map( &mut self )
// 	{	self.map .iter_mut().for_each( | x | x.fill( MapObj::Wall ) );
// 		self.bits.iter_mut().for_each( | x | x.fill( 0            ) );
// 		self.halls= 0;
// 	}

// 	//壁判定 -> true: 壁である、false: 壁ではない
// 	pub fn is_wall( &self, grid: MapGrid ) -> bool
// 	{	if ! RANGE_MAP_X.contains( grid.x() ) || ! RANGE_MAP_Y.contains( grid.y() ) { return true }
// 		matches!( self.mapobj( grid ), MapObj::Wall )
// 	}
// }

// ////////////////////////////////////////////////////////////////////////////////////////////////////

// //自機のComponent
// #[derive(Component)]
// pub struct Player
// {	pub grid: MapGrid,
// 	pub side: DxDy,
// 	pub key_input: DxDy,
// 	pub wait: Timer,
// 	pub stop: bool,
// }

// //追手のComponent
// #[derive(Component)]
// pub struct Chaser
// {	pub grid: MapGrid,
// 	pub side: DxDy,
// 	pub wait: Timer,
// 	pub wandering: Timer,
// 	pub stop: bool,
// 	pub lockon: bool,
// }

//End of code.