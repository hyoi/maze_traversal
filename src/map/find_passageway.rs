use super::*;

//三型迷路：マップを全面走査して、壊すと道を拡張できる壁を探し、壊し尽くすまで壊しまくる
pub fn find_passageway( maze: &mut GameMap )
{	for ( x, ary ) in maze.map.iter().enumerate()
	{	for ( y, _obj ) in ary.iter().enumerate()
		{	if MAP_DIGABLE_X.contains( &( x as i32 ) )
			&& MAP_DIGABLE_Y.contains( &( y as i32 ) )
			&& ! matches!( maze.map[ x ][ y ], MapObj::Wall(_) )
			&& is_passageway( maze, x, y )
			{	maze.stat[ x ][ y ] |= BIT2_PASSAGEWAY;
			}
		}
	}
}

//迷路拡張条件を満たす壁か？
fn is_passageway( maze: &GameMap, x:usize, y:usize ) -> bool
{	let objs = maze.enclosure( x as i32, y as i32 );

	if ! matches!( objs.upper_left  , MapObj::Wall(_) )
	&& ! matches!( objs.upper_center, MapObj::Wall(_) )
	&& ! matches!( objs.middle_left , MapObj::Wall(_) ) { return false }

	if ! matches!( objs.upper_right , MapObj::Wall(_) )
	&& ! matches!( objs.upper_center, MapObj::Wall(_) )
	&& ! matches!( objs.middle_right, MapObj::Wall(_) ) { return false }

	if ! matches!( objs.middle_left , MapObj::Wall(_) )
	&& ! matches!( objs.lower_left  , MapObj::Wall(_) )
	&& ! matches!( objs.lower_center, MapObj::Wall(_) ) { return false }

	if ! matches!( objs.middle_right, MapObj::Wall(_) )
	&& ! matches!( objs.lower_center, MapObj::Wall(_) )
	&& ! matches!( objs.lower_right , MapObj::Wall(_) ) { return false }

	true
}

//End of code.