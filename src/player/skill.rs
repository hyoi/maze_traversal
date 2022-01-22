use super::*;

//Skill関連の変数を格納するResource
pub struct SkillParameters
{	pub auto_mapping: Vec<Vec<(i32,i32)>>,
}

//Skill関連の初期化
impl Default for SkillParameters
{	fn default() -> Self
	{	SkillParameters
		{	auto_mapping: auto_mapping_lv_and_area(),
		}
	}
}

////////////////////////////////////////////////////////////////////////////////////////////////////

//スキル・オートマッピングの初期化
fn auto_mapping_lv_and_area() -> Vec<Vec<(i32,i32)>>
{	//中心(0)を探してその位置を記録する
	let mut center = ( -1, -1 );
	'outer: for ( row, str ) in AUTO_MAPPING_LV_AND_AREA.iter().enumerate()
	{	for ( col, cha ) in str.chars().enumerate()
		{	if cha != '0' { continue }
			center = ( col as i32, row as i32 );
			break 'outer;
		}
	}

	//中心(0)からの差分を記録する
	let mut ary = vec![ Vec::new(); 6 ];
	( 0.. ).zip( AUTO_MAPPING_LV_AND_AREA ).for_each( | ( row, str ) |
	{	( 0.. ).zip( str.chars() ).for_each( | ( col, cha ) |
		{	if let Some ( lv ) = cha.to_digit( 10 )
			{	let dxdy = ( col - center.0, row - center.1 );
				ary[ lv as usize ].push( dxdy )
			}
		} );
	} );

	ary
}

//スキル・オートマッピングの実行。指定されたマスの周囲ｎマスを可視化する
impl GameMap
{	pub fn show_enclosure_obj
	(	&mut self, x: i32, y: i32, skill_lv: usize,
		mut q: Query<&mut Visible>,
		auto_mappiing_lv_and_area: &[ Vec<(i32,i32)> ],
	)
	{	for v_dxdy in auto_mappiing_lv_and_area.iter().take( skill_lv + 1 ) 
		{	for ( dx, dy ) in v_dxdy
			{	self.show( x - dx, y - dy, &mut q );
			}
		}
	}
}

////////////////////////////////////////////////////////////////////////////////////////////////////

//End of code.