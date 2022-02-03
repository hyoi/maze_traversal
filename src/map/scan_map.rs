use super::*;

impl GameMap
{	//広間と通路を区別する
	pub fn distinguish_halls_and_passages( &mut self )
	{	//全面走査して壁以外のマスを調べる
		for x in RANGE_MAP_X
		{	for y in RANGE_MAP_Y
			{	//通路にマークする
				let grid = MapGrid{ x, y };
				if self.is_wall( grid ) { continue } //壁
				if ! self.judge_passageway( grid ) { continue } //広間
				self.set_flag_passageway( grid );

				//上下左右に壁がいくつあるか
				let mut count = 0;
				if self.is_wall_upper_center ( grid ) { count += 1 }
				if self.is_wall_middle_left  ( grid ) { count += 1 }
				if self.is_wall_middle_right ( grid ) { count += 1 }
				if self.is_wall_lower_center ( grid ) { count += 1 }

				//袋小路にマークする
				if count == 3 { self.set_flag_dead_end( grid ); }
			}
		}
	}

	//通路か(true)広間か(false)判断する
	fn judge_passageway( &self, grid: MapGrid ) -> bool
	{	//通路ではない条件
		if ! self.is_wall_upper_left   ( grid ) // XX
		&& ! self.is_wall_upper_center ( grid ) // XO
		&& ! self.is_wall_middle_left  ( grid ) { return false }

		if ! self.is_wall_upper_right  ( grid ) // XX
		&& ! self.is_wall_upper_center ( grid ) // OX
		&& ! self.is_wall_middle_right ( grid ) { return false }

		if ! self.is_wall_middle_left  ( grid ) // XO
		&& ! self.is_wall_lower_left   ( grid ) // XX
		&& ! self.is_wall_lower_center ( grid ) { return false }

		if ! self.is_wall_middle_right ( grid ) // OX
		&& ! self.is_wall_lower_center ( grid ) // XX
		&& ! self.is_wall_lower_right  ( grid ) { return false }

		true
	}

	//行き止まりの路地の長さを数えてコインを設定する
	pub fn length_of_deadend( &mut self )
	{	//全面走査して壁以外のマスを調べる
		for x in RANGE_MAP_INNER_X
		{	for y in RANGE_MAP_INNER_Y
			{	let mut grid = MapGrid{ x, y };
				if ! self.is_dead_end( grid ) { continue }	//袋小路を見つける

				//袋小路から他の道との合流地点まで遡って道の長さを数える
				let mut pedometer = 0;
				let mut back = grid;	//初期値に意味なし
				loop
				{	let mut next = grid;	//初期値に意味なし
					let mut count = 0;
					for dxdy in FOUR_SIDES
					{	let work = grid + dxdy;

						if self.is_wall( work ) || work == back { continue }	//壁 or 自分が来た方向は無視
						next = work;	//先に進める方向を記録
						count += 1;		//先に進める方向を数える
					}
					if count != 1 { break }	//count==1ならnextが先に進める唯一の道

					//道の長さを＋１する
					pedometer += 1;
					back = grid;
					grid = next;
				}
				self.set_coin( MapGrid{ x, y }, pedometer );
			}
		}
	}
}

//End of code.