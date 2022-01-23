//ゲームの状態遷移
#[derive(Clone,Copy,Debug,Eq,PartialEq,Hash)]
pub enum GameState
{	Init,
	Start,
	Play,
	Clear,
	Over,
	Pause
}

//迷路生成関数の選択
#[allow(dead_code)]
#[derive(PartialEq,Debug)]
pub enum SelectMazeType { Random, Type1, Type2, Type3 }

//全体に影響する変数を格納するResource
pub struct SystemParameters
{	pub stage    : usize,
	pub score    : usize,
	pub hp_max   : f32,
	pub hp_now   : f32,
	pub maze_type: SelectMazeType,
	pub darkmode : bool,
	pub sysinfo  : bool,
}
impl Default for SystemParameters
{	fn default() -> Self
	{	Self
		{	stage    : 0,
			score    : 0,
			hp_max   : 100.0,
			hp_now   : 100.0,
			maze_type: SelectMazeType::Random,
			darkmode : false,
			sysinfo  : true,
		}
	}
}

//End of code.