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
		{	//上下左右にある掘削候補と戻り路を記録する
			digable_walls.clear();
			backtrack = ( 0, 0 );
			for ( dx, dy ) in DIRECTION
			{	let tmp_x = map_xy.0 + dx;
				let tmp_y = map_xy.1 + dy;

				//外壁は掘れない
				if ! MAP_DIGABLE_X.contains( &tmp_x ) || ! MAP_DIGABLE_Y.contains( &tmp_y ) { continue }

				//上下左右の座標のオブジェクトを調べる
				let tmp_xy = ( tmp_x, tmp_y );
				let direct = ( dx, dy );
				match self.map[ tmp_x as usize ][ tmp_y as usize ]
				{	MapObj::Dot1 => backtrack = tmp_xy,
					MapObj::Wall(_) if self.is_digable_wall( tmp_xy, direct ) => digable_walls.push( tmp_xy ),
					_ => {}
				}
			}

			//掘れる壁が見つからないか？
			if digable_walls.is_empty()
			{	//戻り路も見つからないなら迷路完成
				if backtrack == ( 0, 0 ) { break }

				//現在位置に行き止まり情報「dot2」を書き込み、後戻りする
				self.map[ map_xy.0 as usize ][ map_xy.1 as usize ] = MapObj::Dot2;
				map_xy = backtrack;
			}
			else
			{	//掘れる壁が見つかったので、方向をランダムに決めて、掘る
				map_xy = digable_walls[ self.rng.gen_range( 0..digable_walls.len() ) ];
				self.map[ map_xy.0 as usize ][ map_xy.1 as usize ] = MapObj::Dot1;
			}
		}

		//三型迷路の作成関数を流用して、道幅拡張工事
		self.find_and_destroy_digable_walls();
	} 

	//進行方向の壁が掘れるか調べる
	fn is_digable_wall( &self, ( x, y ): ( i32, i32 ), direction: ( i32, i32 ) ) -> bool
	{	match direction
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