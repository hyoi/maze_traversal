use super::*;

impl GameMap
{	//二型迷路：ランダムに掘り進み、行き止まりは後戻りして掘れる場所を探す。掘り尽くすまで掘りまくる
	pub fn dig_and_back_and_dig( &mut self )
	{	let mut map = self.start_xy;
		map.y -= 1; //maze.start_xyの直上(y-1)がトンネル掘りの開始座標

		//トンネルを掘る
		let mut digable_walls = Vec::new();
		let mut backtrack;
		loop
		{	digable_walls.clear();
			backtrack = MapGrid { x: 0, y: 0 };

			//上下左右にある掘削候補と戻り路を記録する
			for dxy in FOUR_SIDES
			{	let tmp = MapGrid { x: map.x + dxy.0 - 1, y: map.y + dxy.1 - 1 };

				//外壁は掘れない
				if ! RANGE_MAP_INNER_X.contains( &tmp.x )
				|| ! RANGE_MAP_INNER_Y.contains( &tmp.y ) { continue }

				//上下左右の座標のオブジェクトを調べる
				match self.map[ tmp.x ][ tmp.y ]
				{	MapObj::Pathway => backtrack = tmp,
					MapObj::Wall if self.is_digable_wall( tmp, dxy ) => digable_walls.push( tmp ),
					_ => {}
				}
			}

			//掘れる壁が見つからないか？
			if digable_walls.is_empty()
			{	//戻り路も見つからないなら迷路完成
				if matches!( backtrack, MapGrid { x: 0, y: 0 } ) { break }

				//現在位置に行き止まり情報を書き込み、後戻りする
				self.map[ map.x ][ map.y ] = MapObj::DeadEnd;
				map = backtrack;
			}
			else
			{	//掘れる壁が見つかったので、方向をランダムに決めて、掘る
				map = digable_walls[ self.rng.gen_range( 0..digable_walls.len() ) ];
				self.map[ map.x ][ map.y ] = MapObj::Pathway;
			}
		}

		//三型迷路の作成関数を流用して、道幅拡張工事
		self.find_and_destroy_digable_walls();
	} 

	//進行方向の壁が掘れるか調べる
	fn is_digable_wall( &self, map: MapGrid, four_sides: ( usize, usize ) ) -> bool
	{	match four_sides
		{	UP    if self.is_wall_upper_left   ( map.x, map.y )
				  && self.is_wall_upper_center ( map.x, map.y )	// 壁壁壁
				  && self.is_wall_upper_right  ( map.x, map.y )	// 壁Ｘ壁
				  && self.is_wall_middle_left  ( map.x, map.y )
				  && self.is_wall_middle_right ( map.x, map.y ) => true,
			LEFT  if self.is_wall_upper_left   ( map.x, map.y )	// 壁壁
				  && self.is_wall_upper_center ( map.x, map.y )	// 壁Ｘ
				  && self.is_wall_middle_left  ( map.x, map.y )	// 壁壁
				  && self.is_wall_lower_left   ( map.x, map.y )
				  && self.is_wall_lower_center ( map.x, map.y ) => true,
			RIGHT if self.is_wall_upper_center ( map.x, map.y )	// 壁壁
				  && self.is_wall_upper_right  ( map.x, map.y )	// Ｘ壁
				  && self.is_wall_middle_right ( map.x, map.y )	// 壁壁
				  && self.is_wall_lower_center ( map.x, map.y )
				  && self.is_wall_lower_right  ( map.x, map.y ) => true,
			DOWN  if self.is_wall_middle_left  ( map.x, map.y )
				  && self.is_wall_middle_right ( map.x, map.y )	// 壁Ｘ壁
				  && self.is_wall_lower_left   ( map.x, map.y )	// 壁壁壁
				  && self.is_wall_lower_center ( map.x, map.y )
				  && self.is_wall_lower_right  ( map.x, map.y ) => true,
			_ => { false }
		}
	}
}

//End of code.