use super::*;

impl GameMap
{	//三型迷路：マップを全面走査して、壊すと道を拡張できる壁を探し、壊し尽くすまで壊しまくる
	pub fn find_and_destroy_digable_walls( &mut self )
	{	let mut digable_walls = Vec::new();
		loop
		{	digable_walls.clear();

			//マップを全面走査して拡張条件を満たす壁を記録する
			for x in RANGE_MAP_INNER_X
			{	for y in RANGE_MAP_INNER_Y
				{	if self.is_maze_expandable( x, y ) { digable_walls.push( ( x, y ) ) }
				}
			}

			//条件を満たす壁が見つからなければ迷路完成
			if digable_walls.is_empty() { break }

			//複数候補の中からランダムに壊す壁を決め、道にする
			let ( x, y ) = digable_walls[ self.rng.gen_range( 0..digable_walls.len() ) ];
			self.map[ x ][ y ] = MapObj::Pathway;
		}
	}

	//迷路拡張条件を満たす壁か？
	fn is_maze_expandable( &self, x: usize, y: usize ) -> bool
	{	//そもそも壁ではないので掘れない
		if ! self.is_wall( x, y ) { return false }

		//下向き凸の削り許可
		if   self.is_wall_upper_left   ( x, y )
		&&   self.is_wall_upper_center ( x, y )
		&&   self.is_wall_upper_right  ( x, y )
		&& ! self.is_wall_middle_left  ( x, y )
		&& ! self.is_wall_middle_right ( x, y )
		&& ! self.is_wall_lower_left   ( x, y )
		&& ! self.is_wall_lower_center ( x, y )
		&& ! self.is_wall_lower_right  ( x, y ) { return true }

		//右向き凸の削り許可
		if   self.is_wall_upper_left   ( x, y )
		&& ! self.is_wall_upper_center ( x, y )
		&& ! self.is_wall_upper_right  ( x, y )
		&&   self.is_wall_middle_left  ( x, y )
		&& ! self.is_wall_middle_right ( x, y )
		&&   self.is_wall_lower_left   ( x, y )
		&& ! self.is_wall_lower_center ( x, y )
		&& ! self.is_wall_lower_right  ( x, y ) { return true }

		//左向き凸の削り許可
		if ! self.is_wall_upper_left   ( x, y )
		&& ! self.is_wall_upper_center ( x, y )
		&&   self.is_wall_upper_right  ( x, y )
		&& ! self.is_wall_middle_left  ( x, y )
		&&   self.is_wall_middle_right ( x, y )
		&& ! self.is_wall_lower_left   ( x, y )
		&& ! self.is_wall_lower_center ( x, y )
		&&   self.is_wall_lower_right  ( x, y ) { return true }

		//上向き凸の削り許可
		if ! self.is_wall_upper_left   ( x, y )
		&& ! self.is_wall_upper_center ( x, y )
		&& ! self.is_wall_upper_right  ( x, y )
		&& ! self.is_wall_middle_left  ( x, y )
		&& ! self.is_wall_middle_right ( x, y )
		&&   self.is_wall_lower_left   ( x, y )
		&&   self.is_wall_lower_center ( x, y )
		&&   self.is_wall_lower_right  ( x, y ) { return true }

		//縦の貫通路になる場合はfalse
		if ! self.is_wall_upper_center ( x, y )
		&& ! self.is_wall_lower_center ( x, y ) { return false }

		//横の貫通路になる場合はfalse
		if ! self.is_wall_middle_left  ( x, y )
		&& ! self.is_wall_middle_right ( x, y ) { return false }

		//左上が壁でなく、上と左が壁ならfalse
		if ! self.is_wall_upper_left   ( x, y )
		&&	 self.is_wall_upper_center ( x, y )
		&&	 self.is_wall_middle_left  ( x, y ) { return false }

		//右上が壁でなく、上と右が壁ならfalse
		if ! self.is_wall_upper_right  ( x, y )
		&&	 self.is_wall_upper_center ( x, y )
		&&	 self.is_wall_middle_right ( x, y ) { return false }

		//左下が壁でなく、下と左が壁ならfalse
		if ! self.is_wall_lower_left   ( x, y )
		&&	 self.is_wall_middle_left  ( x, y )
		&&	 self.is_wall_lower_center ( x, y ) { return false }

		//右下が壁でなく、下と右が壁ならfalse
		if ! self.is_wall_lower_right  ( x, y )
		&&	 self.is_wall_middle_right ( x, y )
		&&	 self.is_wall_lower_center ( x, y ) { return false }

		//上下左右がすべて壁はfalse（掘ると飛び地になる）
		if	 self.is_wall_upper_center ( x, y )
		&&	 self.is_wall_middle_left  ( x, y )
		&&	 self.is_wall_middle_right ( x, y )
		&&	 self.is_wall_lower_center ( x, y ) { return false }

		//掘削できる壁
		true
	}
}

//End of code.