use super::*;

//ゲームの状態遷移
#[derive(Clone,Copy,Debug,Eq,PartialEq,Hash)]
pub enum GameState { Init, Start, Play, Event, Clear, Over, Pause }

//全体に影響する変数を格納するResource
pub struct SystemParameters
{	stage    : usize,
	maze_type: SelectMazeType,
	darkmode : bool,
	sysinfo  : bool,
}
impl Default for SystemParameters
{	fn default() -> Self
	{	Self
		{	stage    : 0,
			maze_type: SelectMazeType::Random,
			darkmode : true,
			sysinfo  : false,
		}
	}
}

//End of code.