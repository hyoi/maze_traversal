use super::*;

impl GameMap
{	//三型迷路：マップを全面走査して、壊すと道を拡張できる壁を探し、壊し尽くすまで壊しまくる
	pub fn find_and_destroy_digable_walls( &mut self )
	{	let mut digable_walls = Vec::new();
		loop
		{	//マップを全面走査して拡張条件を満たす壁を記録する
			digable_walls.clear();
			for ( x, ary ) in self.map.iter().enumerate()
			{	for ( y, _obj ) in ary.iter().enumerate()
				{	if MAP_DIGABLE_X.contains( &( x as i32 ) )
					&& MAP_DIGABLE_Y.contains( &( y as i32 ) )
					&& matches!( self.map[ x ][ y ], MapObj::Wall(_) )
					&& self.is_maze_expandable( x, y ) { digable_walls.push( ( x, y ) ) }
				}
			}

			//条件を満たす壁が見つからなければ迷路完成
			if digable_walls.is_empty() { break }

			//複数候補の中からランダムに壊す壁を決め、道にする
			let ( x, y ) = digable_walls[ self.rng.gen_range( 0..digable_walls.len() ) ];
			self.map[ x ][ y ] = MapObj::Dot1( None );
		}
	}

	//迷路拡張条件を満たす壁か？
	fn is_maze_expandable( &self, x:usize, y:usize ) -> bool
	{	let objs = self.enclosure( x as i32, y as i32 );

		//下向き凸の削り許可
		if   matches!( objs.upper_left  , MapObj::Wall(_) )
		&&   matches!( objs.upper_center, MapObj::Wall(_) )
		&&   matches!( objs.upper_right , MapObj::Wall(_) )
		&& ! matches!( objs.middle_left , MapObj::Wall(_) )
		&& ! matches!( objs.middle_right, MapObj::Wall(_) )
		&& ! matches!( objs.lower_left  , MapObj::Wall(_) )
		&& ! matches!( objs.lower_center, MapObj::Wall(_) )
		&& ! matches!( objs.lower_right , MapObj::Wall(_) ) { return true }

		//右向き凸の削り許可
		if   matches!( objs.upper_left  , MapObj::Wall(_) )
		&& ! matches!( objs.upper_center, MapObj::Wall(_) )
		&& ! matches!( objs.upper_right , MapObj::Wall(_) )
		&&   matches!( objs.middle_left , MapObj::Wall(_) )
		&& ! matches!( objs.middle_right, MapObj::Wall(_) )
		&&   matches!( objs.lower_left  , MapObj::Wall(_) )
		&& ! matches!( objs.lower_center, MapObj::Wall(_) )
		&& ! matches!( objs.lower_right , MapObj::Wall(_) ) { return true }

		//左向き凸の削り許可
		if ! matches!( objs.upper_left  , MapObj::Wall(_) )
		&& ! matches!( objs.upper_center, MapObj::Wall(_) )
		&&   matches!( objs.upper_right , MapObj::Wall(_) )
		&& ! matches!( objs.middle_left , MapObj::Wall(_) )
		&&   matches!( objs.middle_right, MapObj::Wall(_) )
		&& ! matches!( objs.lower_left  , MapObj::Wall(_) )
		&& ! matches!( objs.lower_center, MapObj::Wall(_) )
		&&   matches!( objs.lower_right , MapObj::Wall(_) ) { return true }

		//上向き凸の削り許可
		if ! matches!( objs.upper_left  , MapObj::Wall(_) )
		&& ! matches!( objs.upper_center, MapObj::Wall(_) )
		&& ! matches!( objs.upper_right , MapObj::Wall(_) )
		&& ! matches!( objs.middle_left , MapObj::Wall(_) )
		&& ! matches!( objs.middle_right, MapObj::Wall(_) )
		&&   matches!( objs.lower_left  , MapObj::Wall(_) )
		&&   matches!( objs.lower_center, MapObj::Wall(_) )
		&&   matches!( objs.lower_right , MapObj::Wall(_) ) { return true }

		//縦の貫通路になる場合はfalse
		if ! matches!( objs.upper_center, MapObj::Wall(_) )
		&& ! matches!( objs.lower_center, MapObj::Wall(_) ) { return false }

		//横の貫通路になる場合はfalse
		if ! matches!( objs.middle_left , MapObj::Wall(_) )
		&& ! matches!( objs.middle_right, MapObj::Wall(_) ) { return false }

		//左上が壁でなく、上と左が壁ならfalse
		if ! matches!( objs.upper_left  , MapObj::Wall(_) )
		&&	 matches!( objs.upper_center, MapObj::Wall(_) )
		&&	 matches!( objs.middle_left , MapObj::Wall(_) ) { return false }

		//右上が壁でなく、上と右が壁ならfalse
		if ! matches!( objs.upper_right , MapObj::Wall(_) )
		&&	 matches!( objs.upper_center, MapObj::Wall(_) )
		&&	 matches!( objs.middle_right, MapObj::Wall(_) ) { return false }

		//左下が壁でなく、下と左が壁ならfalse
		if ! matches!( objs.lower_left  , MapObj::Wall(_) )
		&&	 matches!( objs.middle_left , MapObj::Wall(_) )
		&&	 matches!( objs.lower_center, MapObj::Wall(_) ) { return false }

		//右下が壁でなく、下と右が壁ならfalse
		if ! matches!( objs.lower_right , MapObj::Wall(_) )
		&&	 matches!( objs.middle_right, MapObj::Wall(_) )
		&&	 matches!( objs.lower_center, MapObj::Wall(_) ) { return false }

		//上下左右がすべて壁はfalse（掘ると飛び地になる）
		if	 matches!( objs.upper_center, MapObj::Wall(_) )
		&&	 matches!( objs.middle_left , MapObj::Wall(_) )
		&&	 matches!( objs.middle_right, MapObj::Wall(_) )
		&&	 matches!( objs.lower_center, MapObj::Wall(_) ) { return false }

		//掘削できる壁
		true
	}
}

//End of code.