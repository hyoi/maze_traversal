use super::*;

//external modules
use std::collections::HashMap;

//ゲームの状態遷移
#[derive(Clone,Copy,Debug,Eq,PartialEq,Hash)]
pub enum GameState
{	Init,
	Start,
	Play,
	Event,
	Clear,
	Over,
	Pause
}

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

////////////////////////////////////////////////////////////////////////////////////////////////////

//Playerの変数を格納するResource
#[allow(dead_code)]
pub struct PlayerParameters
{	name: String,
	flavor_text: String,
	level: usize,
	pub hp_max: f32,
	pub hp_now: f32,
	pub skill_set: HashMap<&'static str, usize>,
//	uuid: UUID,
//	create: Datetime,
//	update: Datetime,
}
impl Default for PlayerParameters
{	fn default() -> Self
	{	//初期取得済みのスキルのレベル
		let mut skill_set = HashMap::new();
		skill_set.insert( SKILL_AUTO_MAPPING, 1 );	//Lv range: 0..=5

		Self
		{	name: "".to_string(),
			flavor_text: "".to_string(),		
			level: 1,
			hp_max: 100.0,
			hp_now: 100.0,
			skill_set,
		}
	}
}

//End of code.