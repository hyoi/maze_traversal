use super::*;

impl GameMap
{	//二型迷路：ランダムに掘り進み、行き止まりは後戻りして掘れる場所を探す。掘り尽くすまで掘りまくる
	pub fn dig_and_back_and_dig( &mut self )
	{	let mut map_xy = self.start_xy;
		map_xy.1 -= 1; //maze.start_xyの直上(y-1)がトンネル掘りの開始座標

		//トンネルを掘る
		let mut digable_walls = Vec::new();
		let mut backtrack;
		loop
		{	digable_walls.clear();
			backtrack = ( 0, 0 );

			//上下左右にある掘削候補と戻り路を記録する
			for dxy in FOUR_SIDES
			{	let tmp_xy = ( map_xy.0 + dxy.0 - 1, map_xy.1 + dxy.1 - 1 );

				//外壁は掘れない
				if ! RANGE_MAP_INNER_X.contains( &tmp_xy.0 )
				|| ! RANGE_MAP_INNER_Y.contains( &tmp_xy.1 ) { continue }

				//上下左右の座標のオブジェクトを調べる
				match self.map[ tmp_xy.0 ][ tmp_xy.1 ]
				{	MapObj::Pathway => backtrack = tmp_xy,
					MapObj::Wall if self.is_digable_wall( tmp_xy, dxy ) => digable_walls.push( tmp_xy ),
					_ => {}
				}
			}

			//掘れる壁が見つからないか？
			if digable_walls.is_empty()
			{	//戻り路も見つからないなら迷路完成
				if backtrack == ( 0, 0 ) { break }

				//現在位置に行き止まり情報を書き込み、後戻りする
				self.map[ map_xy.0 ][ map_xy.1 ] = MapObj::DeadEnd;
				map_xy = backtrack;
			}
			else
			{	//掘れる壁が見つかったので、方向をランダムに決めて、掘る
				map_xy = digable_walls[ self.rng.gen_range( 0..digable_walls.len() ) ];
				self.map[ map_xy.0 ][ map_xy.1 ] = MapObj::Pathway;
			}
		}

		//三型迷路の作成関数を流用して、道幅拡張工事
		self.find_and_destroy_digable_walls();
	} 

	//進行方向の壁が掘れるか調べる
	fn is_digable_wall( &self, ( x, y ): ( usize, usize ), four_sides: ( usize, usize ) ) -> bool
	{	match four_sides
		{	UP    if self.is_wall_upper_left   ( x, y )
				  && self.is_wall_upper_center ( x, y )	// 壁壁壁
				  && self.is_wall_upper_right  ( x, y )	// 壁Ｘ壁
				  && self.is_wall_middle_left  ( x, y )
				  && self.is_wall_middle_right ( x, y ) => true,
			LEFT  if self.is_wall_upper_left   ( x, y )	// 壁壁
				  && self.is_wall_upper_center ( x, y )	// 壁Ｘ
				  && self.is_wall_middle_left  ( x, y )	// 壁壁
				  && self.is_wall_lower_left   ( x, y )
				  && self.is_wall_lower_center ( x, y ) => true,
			RIGHT if self.is_wall_upper_center ( x, y )	// 壁壁
				  && self.is_wall_upper_right  ( x, y )	// Ｘ壁
				  && self.is_wall_middle_right ( x, y )	// 壁壁
				  && self.is_wall_lower_center ( x, y )
				  && self.is_wall_lower_right  ( x, y ) => true,
			DOWN  if self.is_wall_middle_left  ( x, y )
				  && self.is_wall_middle_right ( x, y )	// 壁Ｘ壁
				  && self.is_wall_lower_left   ( x, y )	// 壁壁壁
				  && self.is_wall_lower_center ( x, y )
				  && self.is_wall_lower_right  ( x, y ) => true,
			_ => { false }
		}
	}
}

//End of code.