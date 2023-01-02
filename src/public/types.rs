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
#[derive(Resource)]
pub struct Record
{	pub stage: usize,
	pub score: usize,
	pub hp	 : f32,
}
impl Default for Record
{	fn default() -> Self
	{	Self
		{	stage: 0,
			score: 0,
			hp   : MAX_HP,
		}
	}
}

////////////////////////////////////////////////////////////////////////////////////////////////////

//マーカーResource
#[derive(Resource)]
pub struct DbgOptResUI;

//テキストUIのCompornent
#[derive(Component)]
pub struct MessageClear;

#[derive(Component)]
pub struct MessageOver;

////////////////////////////////////////////////////////////////////////////////////////////////////

//Map用の二次元配列での座標
#[derive(Default,Copy,Clone,PartialEq,Eq)]
pub struct MapGrid { pub x: i32, pub y: i32 }
impl MapGrid
{	//二次元配列の座標から画面座標を算出する
	pub fn into_pixel( self ) -> Pixel
	{	let x = ( PIXEL_PER_GRID - SCREEN_WIDTH  ) / 2.0 + PIXEL_PER_GRID * self.x as f32;
		let y = ( SCREEN_HEIGHT - PIXEL_PER_GRID ) / 2.0 - PIXEL_PER_GRID * self.y as f32 - PIXEL_PER_GRID;
		Pixel { x, y }
	}
}

//四方
#[derive(Copy,Clone,PartialEq,Eq)]
pub enum DxDy { Up, Left, Right, Down, }

//MapGridとDxDyを加算できるようAdd()をオーバーロードする
use std::ops::*;

//MapGrid = MapGrid + DxDy
impl Add<DxDy> for MapGrid
{	type Output = MapGrid;
	fn add( mut self, dxdy: DxDy ) -> MapGrid
	{	match dxdy
		{	DxDy::Up    => { self.y -= 1; }
			DxDy::Left  => { self.x -= 1; }
			DxDy::Right => { self.x += 1; }
			DxDy::Down  => { self.y += 1; }
		}
		self
	}
}
impl Add<&DxDy> for MapGrid
{	type Output = MapGrid;
	fn add( mut self, dxdy: &DxDy ) -> MapGrid
	{	match dxdy
		{	DxDy::Up    => { self.y -= 1; }
			DxDy::Left  => { self.x -= 1; }
			DxDy::Right => { self.x += 1; }
			DxDy::Down  => { self.y += 1; }
		}
		self
	}
}
//MapGrid = DxDy + MapGrid 
impl Add<MapGrid> for DxDy
{	type Output = MapGrid;
	fn add( self, mut grid: MapGrid ) -> MapGrid
	{	match self
		{	DxDy::Up    => { grid.y -= 1; }
			DxDy::Left  => { grid.x -= 1; }
			DxDy::Right => { grid.x += 1; }
			DxDy::Down  => { grid.y += 1; }
		}
		grid
	}
}
impl Add<&MapGrid> for DxDy
{	type Output = MapGrid;
	fn add( self, grid: &MapGrid ) -> MapGrid
	{	let mut ret = *grid;
		match self
		{	DxDy::Up    => { ret.y -= 1; }
			DxDy::Left  => { ret.x -= 1; }
			DxDy::Right => { ret.x += 1; }
			DxDy::Down  => { ret.y += 1; }
		}
		ret
	}
}
//MapGrid += DxDy
impl AddAssign<DxDy> for MapGrid
{	fn add_assign( &mut self, dxdy: DxDy )
	{	match dxdy
		{	DxDy::Up    => { self.y -= 1; }
			DxDy::Left  => { self.x -= 1; }
			DxDy::Right => { self.x += 1; }
			DxDy::Down  => { self.y += 1; }
		}
	}
}
impl AddAssign<&DxDy> for MapGrid
{	fn add_assign( &mut self, dxdy: &DxDy )
	{	match dxdy
		{	DxDy::Up    => { self.y -= 1; }
			DxDy::Left  => { self.x -= 1; }
			DxDy::Right => { self.x += 1; }
			DxDy::Down  => { self.y += 1; }
		}
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
#[derive(Copy,Clone,PartialEq,Eq)]
pub enum MapObj
{	Wall,
	Passage, //通常の道
	DeadEnd, //迷路作成関数のアルゴリズム用（袋小路の目印）
	Coin ( Option<Entity>, usize ),	//スプライトのEntity IDとゴールドの数値
	Goal ( Option<Entity> ),		//スプライトのEntity ID
}

//MAP情報のResource
#[derive(Resource)]
pub struct GameMap
{	rng: rand::prelude::StdRng,	//再現性がある乱数を使いたいので
	map  : [ [ MapObj; MAP_HEIGHT as usize ]; MAP_WIDTH as usize ],
	bits : [ [ usize ; MAP_HEIGHT as usize ]; MAP_WIDTH as usize ],
	start: MapGrid,
	goal : MapGrid,
	halls: usize,	//広間のマス数
}
impl Default for GameMap
{	fn default() -> Self
	{	//開発で迷路作成に再現性が必要な場合、乱数シードを固定する。本番はランダムにする。
		let seed = if cfg!( debug_assertions ) { 1234567890 } else { rand::thread_rng().gen::<u64>() };

		Self
		{	rng  : StdRng::seed_from_u64( seed ),	
			map  : [ [ MapObj::Wall; MAP_HEIGHT as usize ]; MAP_WIDTH as usize ],
			bits : [ [ 0           ; MAP_HEIGHT as usize ]; MAP_WIDTH as usize ],
			start: MapGrid::default(),
			goal : MapGrid::default(),
			halls: 0,
		}
	}
}

impl GameMap
{	//GameMap構造体のアクセサ
	pub fn rng( &mut self ) -> &mut rand::prelude::StdRng { &mut self.rng }

	pub fn mapobj( &self, grid: MapGrid ) -> MapObj { self.map [ grid.x as usize ][ grid.y as usize ] }
	pub fn mapobj_mut( &mut self, grid: MapGrid ) -> &mut MapObj { &mut self.map[ grid.x as usize ][ grid.y as usize ] }

	pub fn bits( &self, grid: MapGrid ) -> usize { self.bits[ grid.x as usize ][ grid.y as usize ] }
	fn bits_mut( &mut self, grid: MapGrid ) -> &mut usize { &mut self.bits[ grid.x as usize ][ grid.y as usize ] }

	pub fn start( &self ) -> MapGrid { self.start }
	pub fn start_mut( &mut self ) -> &mut MapGrid { &mut self.start }

//	pub fn goal( &self ) -> MapGrid { self.goal }
	pub fn goal_mut( &mut self ) -> &mut MapGrid { &mut self.goal }

//	pub fn halls( &self ) -> usize { self.halls }
	pub fn halls_mut( &mut self ) -> &mut usize { &mut self.halls }

	//指定されたマスのフラグ操作
//	pub fn is_hall   ( &self, grid: MapGrid ) -> bool { self.bits( grid ) & BIT_HALL    != 0 }
//	pub fn is_passage( &self, grid: MapGrid ) -> bool { self.bits( grid ) & BIT_PASSAGE != 0 }
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

//自機のComponent
#[derive(Component)]
pub struct Player
{	pub grid: MapGrid,
	pub side: DxDy,
	pub key_input: DxDy,
	pub wait: Timer,
	pub stop: bool,
}

//追手のComponent
#[derive(Component)]
pub struct Chaser
{	pub grid: MapGrid,
	pub side: DxDy,
	pub wait: Timer,
	pub wandering: Timer,
	pub stop: bool,
	pub lockon: bool,
}

//End of code.