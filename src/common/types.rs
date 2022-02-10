use super::*;

//ゲームの状態遷移
#[allow(dead_code)]
#[derive(Clone,Copy,Debug,Eq,PartialEq,Hash)]
pub enum GameState
{	Init,
	Start,
	Play,
	Clear,
	Over,
	Pause,
	DemoStart,
	DemoPlay,
	DemoLoop,
}

//スコア等のResource
pub struct Record
{	pub stage: usize,
	pub score: usize,
	pub hp	 : f32,
}
impl Default for Record
{	fn default() -> Self { Self { stage: 0, score: 0, hp: MAX_HP, } }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

//マーカーResource
pub struct DbgOptResUI;

////////////////////////////////////////////////////////////////////////////////////////////////////

//Map用の二次元配列での座標
#[derive(Default,Copy,Clone,PartialEq,Eq)]
pub struct MapGrid { pub x: usize, pub y: usize }
impl MapGrid
{	//二次元配列の座標から画面座標を算出する
	pub fn into_pixel( self ) -> Pixel
	{	let x = ( PIXEL_PER_GRID - SCREEN_WIDTH  ) / 2.0 + PIXEL_PER_GRID * self.x as f32;
		let y = ( SCREEN_HEIGHT - PIXEL_PER_GRID ) / 2.0 - PIXEL_PER_GRID * self.y as f32 - PIXEL_PER_GRID;
		Pixel { x, y }
	}
}

//方向（上下左右）の座標増分
#[derive(Default,Copy,Clone,PartialEq,Eq)]
pub struct DxDy { pub dx: i32, pub dy: i32 }

//MapGridとDxDyを加算できるようAdd()をオーバーロードする
use std::ops::*;

//MapGrid = MapGrid + DxDy
impl Add<DxDy> for MapGrid
{	type Output = MapGrid;
	fn add( self, dxdy: DxDy ) -> MapGrid
	{	let x = ( self.x as i32 + dxdy.dx ) as usize;
		let y = ( self.y as i32 + dxdy.dy ) as usize;
		MapGrid { x, y }
	}
}
impl Add<&DxDy> for MapGrid
{	type Output = MapGrid;
	fn add( self, dxdy: &DxDy ) -> MapGrid
	{	let x = ( self.x as i32 + dxdy.dx ) as usize;
		let y = ( self.y as i32 + dxdy.dy ) as usize;
		MapGrid { x, y }
	}
}
//MapGrid = DxDy + MapGrid 
impl Add<MapGrid> for DxDy
{	type Output = MapGrid;
	fn add( self, grid: MapGrid ) -> MapGrid
	{	let x = ( grid.x as i32 + self.dx ) as usize;
		let y = ( grid.y as i32 + self.dy ) as usize;
		MapGrid { x, y }
	}
}
impl Add<&MapGrid> for DxDy
{	type Output = MapGrid;
	fn add( self, grid: &MapGrid ) -> MapGrid
	{	let x = ( grid.x as i32 + self.dx ) as usize;
		let y = ( grid.y as i32 + self.dy ) as usize;
		MapGrid { x, y }
	}
}
//MapGrid += DxDy
impl AddAssign<DxDy> for MapGrid
{	fn add_assign( &mut self, dxdy: DxDy )
	{	self.x = ( self.x as i32 + dxdy.dx ) as usize;
		self.y = ( self.y as i32 + dxdy.dy ) as usize;
 	}
}
impl AddAssign<&DxDy> for MapGrid
{	fn add_assign( &mut self, dxdy: &DxDy )
	{	self.x = ( self.x as i32 + dxdy.dx ) as usize;
		self.y = ( self.y as i32 + dxdy.dy ) as usize;
 	}
}

//スプライト等の画面座標
#[derive(Default,Copy,Clone,PartialEq)]
pub struct Pixel { pub x: f32, pub y: f32 }

//Pixel += ( f32, f32 )
impl AddAssign<(f32,f32)> for Pixel
{	fn add_assign( &mut self, xd: (f32,f32) )
	{	self.x = self.x + xd.0;
		self.y = self.y + xd.1;
 	}
}
impl AddAssign<&(f32,f32)> for Pixel
{	fn add_assign( &mut self, xy: &(f32,f32) )
	{	self.x = self.x + xy.0;
		self.y = self.y + xy.1;
 	}
}

////////////////////////////////////////////////////////////////////////////////////////////////////

//MAPのマスの種類
#[derive(Copy,Clone,PartialEq)]
pub enum MapObj
{	Wall,
	Passage, //通常の道
	DeadEnd, //迷路作成関数のアルゴリズム用（袋小路の目印）
	Coin ( Option<Entity>, usize ),	//スプライトのEntity IDとゴールドの数値
	Goal ( Option<Entity> ),		//スプライトのEntity ID
}

//MAP情報のResource
pub struct GameMap
{	rng: rand::prelude::StdRng,	//再現性がある乱数を使いたいので
	map  : [ [ MapObj; MAP_HEIGHT ]; MAP_WIDTH ],
	bits : [ [ usize ; MAP_HEIGHT ]; MAP_WIDTH ],
	start: MapGrid,
	goal : MapGrid,
	halls: usize,	//広間のマス数
}
impl Default for GameMap
{	fn default() -> Self
	{	Self
		{//	rng: StdRng::seed_from_u64( rand::thread_rng().gen::<u64>() ),	//本番用
			rng: StdRng::seed_from_u64( 1234567890 ),	//開発用：再現性がある乱数を使いたい場合
			map  : [ [ MapObj::Wall; MAP_HEIGHT ]; MAP_WIDTH ],
			bits : [ [ 0			  ; MAP_HEIGHT ]; MAP_WIDTH ],
			start: MapGrid::default(),
			goal : MapGrid::default(),
			halls: 0,
		}
	}
}

impl GameMap
{	//GameMap構造体のアクセサ
	pub fn rng( &mut self ) -> &mut rand::prelude::StdRng { &mut self.rng }

	pub fn mapobj( &self, grid: MapGrid ) -> MapObj { self.map [ grid.x ][ grid.y ] }
	pub fn mapobj_mut( &mut self, grid: MapGrid ) -> &mut MapObj { &mut self.map[ grid.x ][ grid.y ] }

	pub fn bits( &self, grid: MapGrid ) -> usize { self.bits[ grid.x ][ grid.y ] }
	fn bits_mut( &mut self, grid: MapGrid ) -> &mut usize { &mut self.bits[ grid.x ][ grid.y ] }

	pub fn start( &self ) -> MapGrid { self.start }
	pub fn start_mut( &mut self ) -> &mut MapGrid { &mut self.start }

	pub fn goal( &self ) -> MapGrid { self.goal }
	pub fn goal_mut( &mut self ) -> &mut MapGrid { &mut self.goal }

	pub fn halls( &self ) -> usize { self.halls }
	pub fn halls_mut( &mut self ) -> &mut usize { &mut self.halls }

	//指定されたマスのフラグ操作
	pub fn is_hall   ( &self, grid: MapGrid ) -> bool { self.bits( grid ) & BIT_HALL    != 0 }
	pub fn is_passage( &self, grid: MapGrid ) -> bool { self.bits( grid ) & BIT_PASSAGE != 0 }
	pub fn is_deadend( &self, grid: MapGrid ) -> bool { self.bits( grid ) & BIT_DEADEND != 0 }
	pub fn set_flag_hall   ( &mut self, grid: MapGrid ) { *self.bits_mut( grid ) |= BIT_HALL    }
	pub fn set_flag_passage( &mut self, grid: MapGrid ) { *self.bits_mut( grid ) |= BIT_PASSAGE }
	pub fn set_flag_deadend( &mut self, grid: MapGrid ) { *self.bits_mut( grid ) |= BIT_DEADEND }

	//配列を初期化する
	pub fn clear_map( &mut self )
	{	self.map .iter_mut().for_each( | x | x.fill( MapObj::Wall ) );
		self.bits.iter_mut().for_each( | x | x.fill( 0            ) );
		self.halls= 0;
	}

	//壁判定 -> true: 壁である、false: 壁ではない
	pub fn is_wall( &self, grid: MapGrid ) -> bool
	{	if ! RANGE_MAP_X.contains( &grid.x ) || ! RANGE_MAP_Y.contains( &grid.y ) { return true }
		matches!( self.mapobj( grid ), MapObj::Wall )
	}
}

////////////////////////////////////////////////////////////////////////////////////////////////////

//向きを表す列挙型
#[derive(Clone,Copy,PartialEq)]
pub enum FourSides { Up, Left, Right, Down }

impl FourSides
{	pub fn is_up   ( &self ) -> bool { matches!( self, FourSides::Up    ) }
	pub fn is_left ( &self ) -> bool { matches!( self, FourSides::Left  ) }
	pub fn is_right( &self ) -> bool { matches!( self, FourSides::Right ) }
	pub fn is_down ( &self ) -> bool { matches!( self, FourSides::Down  ) }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

//自機のComponent
#[derive(Component)]
pub struct Player
{	pub grid: MapGrid,
	pub side: FourSides,
	pub key_input: FourSides,
	pub wait: Timer,
	pub stop: bool,
}

//追手のComponent
#[derive(Component)]
pub struct Chaser
{	pub grid: MapGrid,
	pub pixel: Pixel,
	// pub pixel_old: Pixel,
	pub side: FourSides,
	pub wait: Timer,
	pub wandering: Timer,
	pub stop: bool,
	pub lockon: bool,
	// pub collision: bool,
	// pub speedup: f32,
}

//End of code.