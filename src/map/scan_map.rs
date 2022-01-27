use super::*;

impl GameMap
{	//広間と通路を区別する
	pub fn distinguish_halls_and_passages( &mut self )
	{	//全面走査して壁以外のマスを調べる
		for x in RANGE_MAP_X
		{	for y in RANGE_MAP_Y
			{	//通路にマークする
				if self.is_wall( x, y ) { continue } //壁
				if ! self.judge_passageway( x, y ) { continue } //広間
				self.set_flag_passageway( x, y );

				//上下左右に壁がいくつあるか
				let mut count = 0;
				if self.is_wall_upper_center ( x, y ) { count += 1 }
				if self.is_wall_middle_left  ( x, y ) { count += 1 }
				if self.is_wall_middle_right ( x, y ) { count += 1 }
				if self.is_wall_lower_center ( x, y ) { count += 1 }

				//袋小路にマークする
				if count == 3 { self.set_flag_dead_end( x, y ); }
			}
		}
	}

	//通路か(true)広間か(false)判断する
	fn judge_passageway( &self, x: usize, y: usize ) -> bool
	{	//通路ではない条件
		if ! self.is_wall_upper_left   ( x, y ) // XX
		&& ! self.is_wall_upper_center ( x, y ) // XO
		&& ! self.is_wall_middle_left  ( x, y ) { return false }

		if ! self.is_wall_upper_right  ( x, y ) // XX
		&& ! self.is_wall_upper_center ( x, y ) // OX
		&& ! self.is_wall_middle_right ( x, y ) { return false }

		if ! self.is_wall_middle_left  ( x, y ) // XO
		&& ! self.is_wall_lower_left   ( x, y ) // XX
		&& ! self.is_wall_lower_center ( x, y ) { return false }

		if ! self.is_wall_middle_right ( x, y ) // OX
		&& ! self.is_wall_lower_center ( x, y ) // XX
		&& ! self.is_wall_lower_right  ( x, y ) { return false }

		true
	}

	//行き止まりの路地の長さを数える
	pub fn length_of_deadend( &mut self )
	{	//全面走査して壁以外のマスを調べる
		for x in RANGE_MAP_INNER_X
		{	for y in RANGE_MAP_INNER_Y
			{	//袋小路を見つける
				if ! self.is_dead_end( x, y ) { continue }
	
				//袋小路から他の道との合流地点まで遡って道の長さを数える
				let mut pedometer = 0;
				let mut map_xy = ( x, y );
				let mut old_xy = map_xy;	//初期値に意味なし
				loop
				{	let mut next_xy = map_xy;	//初期値に意味なし
					let mut count = 0;
					for ( dx, dy ) in FOUR_SIDES
					{	let tmp_xy = ( map_xy.0 + dx - 1, map_xy.1 + dy - 1 );
						if self.is_wall( tmp_xy.0, tmp_xy.1 ) { continue }	//壁なら無視
						if tmp_xy == old_xy { continue }	//自分が来た方向は無視
						next_xy = tmp_xy;	//進める方向を記録
						count += 1;			//進める方向を数える
					}
					if count != 1 { break }	//進める方向が複数あるなら広間か別の道と合流した

					//道の長さを＋１する
					pedometer += 1;
					old_xy = map_xy;
					map_xy = next_xy;
				}
				self.count[ x as usize ][ y as usize ] = pedometer;
			}
		}
	}
}

//End of code.