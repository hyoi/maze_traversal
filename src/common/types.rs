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

//デバッグ用のマーカーResource
pub struct DbgPluginUi;

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
use std::ops;
//MapGrid = MapGrid + DxDy
impl ops::Add<DxDy> for MapGrid
{	type Output = MapGrid;
	fn add( self, dxdy: DxDy ) -> MapGrid
	{	let x = ( self.x as i32 + dxdy.dx ) as usize;
		let y = ( self.y as i32 + dxdy.dy ) as usize;
		MapGrid { x, y }
	}
}
//MapGrid = DxDy + MapGrid 
impl ops::Add<MapGrid> for DxDy
{	type Output = MapGrid;
	fn add( self, grid: MapGrid ) -> MapGrid
	{	let x = ( grid.x as i32 + self.dx ) as usize;
		let y = ( grid.y as i32 + self.dy ) as usize;
		MapGrid { x, y }
	}
}

//スプライト等の画面座標
#[derive(Default,Copy,Clone,PartialEq)]
pub struct Pixel { pub x: f32, pub y: f32 }

////////////////////////////////////////////////////////////////////////////////////////////////////

//MAPのマスの種類
#[derive(Copy,Clone,PartialEq)]
pub enum MapObj
{	Wall,
	Pathway, //通常の道
	DeadEnd, //行き止まり目印用
	Coin ( Option<Entity> ),
	Goal ( Option<Entity> ),
}

//MAP情報のResource
pub struct GameMap
{	pub rng: rand::prelude::StdRng,	//再現性がある乱数を使いたいので
	map : [ [ MapObj; MAP_HEIGHT ]; MAP_WIDTH ],
	bits: [ [ usize ; MAP_HEIGHT ]; MAP_WIDTH ],
	coin: [ [ usize ; MAP_HEIGHT ]; MAP_WIDTH ],
	pub start_xy: MapGrid,
	pub goal_xy : MapGrid,
}
impl Default for GameMap
{	fn default() -> Self
	{	Self
		{//	rng: StdRng::seed_from_u64( rand::thread_rng().gen::<u64>() ),	//本番用
			rng: StdRng::seed_from_u64( 1234567890 ),	//開発用：再現性がある乱数を使いたい場合
			map : [ [ MapObj::Wall; MAP_HEIGHT ]; MAP_WIDTH ],
			bits: [ [ 0			  ; MAP_HEIGHT ]; MAP_WIDTH ],
			coin: [ [ 0			  ; MAP_HEIGHT ]; MAP_WIDTH ],
			start_xy: MapGrid::default(),
			goal_xy : MapGrid::default(),
		}
	}
}

impl GameMap
{	//配列を初期化する
	pub fn clear_map( &mut self )
	{	self.map .iter_mut().for_each( | x | x.fill( MapObj::Wall ) );
		self.bits.iter_mut().for_each( | x | x.fill( 0            ) );
		self.coin.iter_mut().for_each( | x | x.fill( 0            ) );
	}

	//配列の値を返す
	pub fn map ( &self, grid: MapGrid ) -> MapObj { self.map [ grid.x ][ grid.y ] }
	pub fn bits( &self, grid: MapGrid ) -> usize  { self.bits[ grid.x ][ grid.y ] }
	pub fn coin( &self, grid: MapGrid ) -> usize  { self.coin[ grid.x ][ grid.y ] }

	//配列の値をセットする
	pub fn set_mapobj( &mut self, grid: MapGrid, obj : MapObj ) { self.map [ grid.x ][ grid.y ] = obj  }
	pub fn set_coin  ( &mut self, grid: MapGrid, coin: usize  ) { self.coin[ grid.x ][ grid.y ] = coin }

	//指定されたマスのフラグを立てる
	pub fn set_flag_passageway ( &mut self, grid: MapGrid ) { self.bits[ grid.x ][ grid.y ] |= BIT_PASSAGEWAY; }
	pub fn set_flag_dead_end   ( &mut self, grid: MapGrid ) { self.bits[ grid.x ][ grid.y ] |= BIT_DEAD_END;   }
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
{	pub map_xy   : MapGrid,
	pub direction: FourSides,
	pub key_input: FourSides,
	pub wait: Timer,
	pub stop: bool,
}

//追手のComponent
#[derive(Component)]
pub struct Chaser
{	pub map_xy: MapGrid,
	pub pixel_xy: Pixel,
	pub pixel_xy_old: Pixel,
	pub direction: FourSides,
	pub wait: Timer,
	pub wandering: Timer,
	pub stop: bool,
	pub collision: bool,
	pub speedup: f32,
}

//End of code.