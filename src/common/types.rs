use super::*;

//external modules
use rand::prelude::*;

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

////////////////////////////////////////////////////////////////////////////////////////////////////

//Resource
pub struct Record
{	pub stage    : usize,
	pub score    : usize,
	pub hp_max   : f32,
	pub hp_now   : f32,
}
impl Default for Record
{	fn default() -> Self
	{	Self
		{	stage    : 0,
			score    : 0,
			hp_max   : 100.0,
			hp_now   : 100.0,
		}
	}
}

////////////////////////////////////////////////////////////////////////////////////////////////////

//MAPのマスの種類
#[derive(Copy,Clone,PartialEq)]
pub enum MapObj
{	None,
	Wall,
	Pathway, //通常の道
	DeadEnd, //行き止まり目印用
	Coin ( Option<Entity> ),
	Goal ( Option<Entity> ),
	Space,
}

//MAP情報のResource
pub struct GameMap
{	pub rng: rand::prelude::StdRng,	//再現性がある乱数を使いたいので
	pub map  : [ [ MapObj; MAP_HEIGHT ]; MAP_WIDTH ],
	pub bits : [ [ usize ; MAP_HEIGHT ]; MAP_WIDTH ],
	pub count: [ [ usize ; MAP_HEIGHT ]; MAP_WIDTH ],
	pub start_xy: ( usize, usize ),
	pub goal_xy : ( usize, usize ),
}
impl Default for GameMap
{	fn default() -> Self
	{	Self
		{//	rng: StdRng::seed_from_u64( rand::thread_rng().gen::<u64>() ),	//本番用
			rng: StdRng::seed_from_u64( 1234567890 ),	//開発用：再現性がある乱数を使いたい場合
			map  : [ [ MapObj::None ; MAP_HEIGHT ]; MAP_WIDTH ],
			bits : [ [ 0; MAP_HEIGHT ]; MAP_WIDTH ], //BIT_ALL_CLEAR
			count: [ [ 0; MAP_HEIGHT ]; MAP_WIDTH ],
			start_xy: ( 0, 0 ),
			goal_xy : ( 0, 0 ),
		}
	}
}

////////////////////////////////////////////////////////////////////////////////////////////////////

//向きを表す列挙型
#[derive(Clone,Copy,PartialEq)]
pub enum Direction
{	Up,
	Left,
	Right,
	Down,
}

//自機のComponent
#[derive(Component)]
pub struct Player
{	pub wait: Timer,
	pub map_postion: ( usize, usize ),
	pub sprite_postion: ( f32, f32 ),
	pub direction: Direction,
	pub new_direction: Direction,
	pub stop: bool,
}
impl Default for Player
{	fn default() -> Self
	{	Self
		{	wait: Timer::from_seconds( PLAYER_WAIT, false ),
			map_postion: ( 0, 0 ),
			sprite_postion: ( 0.0, 0.0 ),
			direction: Direction::Up,
			new_direction: Direction::Up,
			stop: true,
		}
	}
}

//追手のComponent
#[derive(Component)]
pub struct Chaser
{	pub map_position: ( usize, usize ),
	pub pixel_position: ( f32, f32 ),
	pub pixel_position_old: ( f32, f32 ),
	pub direction: Direction,
	pub wait: Timer,
	pub stop: bool,
	pub collision: bool,
	pub speedup: f32,
}

//End of code.