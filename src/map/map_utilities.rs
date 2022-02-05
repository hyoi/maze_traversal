use super::*;

impl GameMap
{	//広間と通路を識別して袋小路に目印を付ける
	pub fn identify_halls_and_passages( &mut self )
	{	for x in RANGE_MAP_X
		{	for y in RANGE_MAP_Y
			{	let grid = MapGrid{ x, y };
				if self.is_wall( grid ) { continue }

				//マークする
				if self.judge_halls( grid )
				{	//広間
					self.set_flag_hall( grid );					
				}
				else
				{	//通路
					self.set_flag_passage( grid );

					//上下左右の壁を数える
					let mut count = 0;
					FOUR_SIDES.iter().for_each( | x | if self.is_wall( grid + x ) { count += 1 } );

					//袋小路に目印を付ける
					if count == 3 { self.set_flag_deadend( grid ); }
				}
			}
		}
	}

	//広間か(true)、通路か(false)、判断する
	fn judge_halls( &self, grid: MapGrid ) -> bool
	{	if ! self.is_wall( grid + UP   + LEFT  ) // XX
		&& ! self.is_wall( grid + UP           ) // XO
		&& ! self.is_wall( grid        + LEFT  ) { return true }

		if ! self.is_wall( grid + UP   + RIGHT ) // XX
		&& ! self.is_wall( grid + UP           ) // OX
		&& ! self.is_wall( grid        + RIGHT ) { return true }

		if ! self.is_wall( grid        + LEFT  ) // XO
		&& ! self.is_wall( grid + DOWN + LEFT  ) // XX
		&& ! self.is_wall( grid + DOWN         ) { return true }

		if ! self.is_wall( grid        + RIGHT ) // OX
		&& ! self.is_wall( grid + DOWN         ) // XX
		&& ! self.is_wall( grid + DOWN + RIGHT ) { return true }

		false
	}

	//袋小路の路地の長さを測ってコインを置く
	pub fn put_coins_at_deadend( &mut self )
	{	for x in RANGE_MAP_INNER_X
		{	for y in RANGE_MAP_INNER_Y
			{	let mut grid = MapGrid{ x, y };
				if ! self.is_deadend( grid ) { continue }

				//袋小路を起点に他の道との合流地点まで遡って道の長さを数える
				let mut pedometer = 0;	//歩数計
				let mut back = grid;
				loop
				{	let mut next = grid;	//初期値に意味なし
					let mut count = 0;
					for dxdy in FOUR_SIDES
					{	let side = grid + dxdy;
						if self.is_wall( side ) || side == back { continue }	//壁／来た道は無視

						next = side;	//先に進める方向を記録（まだここでは候補）
						count += 1;		//先に進める方向を数える
					}
					if count != 1 { break }	//count > 1なら他の道に合流した

					//道の長さを＋１する
					pedometer += 1;
					back = grid;
					grid = next;
				}

				//袋小路の深さに準じてコインを置く
				*self.mapobj_mut( MapGrid{ x, y } ) = MapObj::Coin ( None, pedometer );
			}
		}
	}
}

//End of code.