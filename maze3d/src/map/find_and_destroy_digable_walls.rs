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
				{	let grid = MapGrid::new( x, y );
					if self.is_maze_expandable( grid ) { digable_walls.push( grid ) }
				}
			}

			//条件を満たす壁が見つからなければ迷路完成
			if digable_walls.is_empty() { break }

			//複数候補の中からランダムに壊す壁を決め、道にする
			let grid = digable_walls[ self.rng().gen_range( 0..digable_walls.len() ) ];
			*self.mapobj_mut( grid ) = MapObj::Passage;
		}
	}

	//迷路拡張条件を満たす壁か？
	fn is_maze_expandable( &self, grid: MapGrid ) -> bool
	{	//そもそも壁ではないので掘れない
		if ! self.is_wall( grid ) { return false }

		//下向き凸の削り許可
		if   self.is_wall( grid + UP + LEFT )
		&&   self.is_wall( grid + UP )
		&&   self.is_wall( grid + UP + RIGHT )
		&& ! self.is_wall( grid + LEFT )
		&& ! self.is_wall( grid + RIGHT )
		&& ! self.is_wall( grid + DOWN + LEFT )
		&& ! self.is_wall( grid + DOWN )
		&& ! self.is_wall( grid + DOWN + RIGHT ) { return true }

		//右向き凸の削り許可
		if   self.is_wall( grid + UP + LEFT )
		&& ! self.is_wall( grid + UP )
		&& ! self.is_wall( grid + UP + RIGHT )
		&&   self.is_wall( grid + LEFT )
		&& ! self.is_wall( grid + RIGHT )
		&&   self.is_wall( grid + DOWN + LEFT )
		&& ! self.is_wall( grid + DOWN )
		&& ! self.is_wall( grid + DOWN + RIGHT ) { return true }

		//左向き凸の削り許可
		if ! self.is_wall( grid + UP + LEFT )
		&& ! self.is_wall( grid + UP )
		&&   self.is_wall( grid + UP + RIGHT )
		&& ! self.is_wall( grid + LEFT )
		&&   self.is_wall( grid + RIGHT )
		&& ! self.is_wall( grid + DOWN + LEFT )
		&& ! self.is_wall( grid + DOWN )
		&&   self.is_wall( grid + DOWN + RIGHT ) { return true }

		//上向き凸の削り許可
		if ! self.is_wall( grid + UP + LEFT )
		&& ! self.is_wall( grid + UP )
		&& ! self.is_wall( grid + UP + RIGHT )
		&& ! self.is_wall( grid + LEFT )
		&& ! self.is_wall( grid + RIGHT )
		&&   self.is_wall( grid + DOWN + LEFT )
		&&   self.is_wall( grid + DOWN )
		&&   self.is_wall( grid + DOWN + RIGHT ) { return true }

		//縦の貫通路になる場合はfalse
		if ! self.is_wall( grid + UP )
		&& ! self.is_wall( grid + DOWN ) { return false }

		//横の貫通路になる場合はfalse
		if ! self.is_wall( grid + LEFT )
		&& ! self.is_wall( grid + RIGHT ) { return false }

		//左上が壁でなく、上と左が壁ならfalse
		if ! self.is_wall( grid + UP + LEFT )
		&&	 self.is_wall( grid + UP )
		&&	 self.is_wall( grid + LEFT ) { return false }

		//右上が壁でなく、上と右が壁ならfalse
		if ! self.is_wall( grid + UP + RIGHT )
		&&	 self.is_wall( grid + UP )
		&&	 self.is_wall( grid + RIGHT ) { return false }

		//左下が壁でなく、下と左が壁ならfalse
		if ! self.is_wall( grid + DOWN + LEFT )
		&&	 self.is_wall( grid + LEFT )
		&&	 self.is_wall( grid + DOWN ) { return false }

		//右下が壁でなく、下と右が壁ならfalse
		if ! self.is_wall( grid + DOWN + RIGHT )
		&&	 self.is_wall( grid + RIGHT )
		&&	 self.is_wall( grid + DOWN ) { return false }

		//上下左右がすべて壁はfalse（掘ると飛び地になる）
		if	 self.is_wall( grid + UP    )
		&&	 self.is_wall( grid + LEFT  )
		&&	 self.is_wall( grid + RIGHT )
		&&	 self.is_wall( grid + DOWN  ) { return false }

		//掘削できる壁
		true
	}
}

//End of code.