use super::*;

impl GameMap
{	//二型迷路：ランダムに掘り進み、行き止まりは後戻りして掘れる場所を探す。掘り尽くすまで掘りまくる
	pub fn dig_and_back_and_dig( &mut self )
	{	let mut grid = self.start();
		grid.y -= 1; //maze.start_xyの直上(y-1)がトンネル掘りの開始座標

		//トンネルを掘る
		let mut digable_walls = Vec::new();
		let mut backtrack;
		loop
		{	digable_walls.clear();
			backtrack = MapGrid { x: 0, y: 0 };

			//上下左右にある掘削候補と戻り路を記録する
			for dxdy in FOUR_SIDES
			{	let next = grid + dxdy;

				//外壁は掘れない
				if ! RANGE_MAP_INNER_X.contains( &next.x )
				|| ! RANGE_MAP_INNER_Y.contains( &next.y ) { continue }

				//上下左右の座標のオブジェクトを調べる
				match self.mapobj( next )
				{	MapObj::Passage => backtrack = next,
					MapObj::Wall if self.is_digable_wall( next, dxdy ) => digable_walls.push( next ),
					_ => {}
				}
			}

			//掘れる壁が見つからないか？
			if digable_walls.is_empty()
			{	//戻り路も見つからないなら迷路完成
				if matches!( backtrack, MapGrid { x: 0, y: 0 } ) { break }

				//現在位置に行き止まり情報を書き込み、後戻りする
				*self.mapobj_mut( grid ) = MapObj::DeadEnd;
				grid = backtrack;
			}
			else
			{	//掘れる壁が見つかったので、方向をランダムに決めて、掘る
				grid = digable_walls[ self.rng().gen_range( 0..digable_walls.len() ) ];
				*self.mapobj_mut( grid ) = MapObj::Passage;
			}
		}

		//三型迷路の作成関数を流用して、道幅拡張工事
		self.find_and_destroy_digable_walls();
	} 

	//進行方向の壁が掘れるか調べる
	fn is_digable_wall( &self, grid: MapGrid, four_sides: DxDy ) -> bool
	{	let mut digable = false;
		match four_sides
		{	UP    => if self.is_wall( grid + UP + LEFT	  )
					 && self.is_wall( grid + UP			  ) // 壁壁壁
					 && self.is_wall( grid + UP + RIGHT   ) // 壁Ｘ壁
					 && self.is_wall( grid + LEFT		  )
					 && self.is_wall( grid + RIGHT		  ) { digable = true },
			LEFT  => if self.is_wall( grid + UP + LEFT	  ) // 壁壁
					 && self.is_wall( grid + UP			  ) // 壁Ｘ
					 && self.is_wall( grid + LEFT		  ) // 壁壁
					 && self.is_wall( grid + DOWN + LEFT  )
					 && self.is_wall( grid + DOWN		  ) { digable = true },
			RIGHT => if self.is_wall( grid + UP			  ) // 壁壁
					 && self.is_wall( grid + UP + RIGHT   ) // Ｘ壁
					 && self.is_wall( grid + RIGHT		  ) // 壁壁
					 && self.is_wall( grid + DOWN 		  )
					 && self.is_wall( grid + DOWN + RIGHT ) { digable = true },
			DOWN  => if self.is_wall( grid + LEFT		  )
					 && self.is_wall( grid + RIGHT		  ) // 壁Ｘ壁
					 && self.is_wall( grid + DOWN + LEFT  ) // 壁壁壁
					 && self.is_wall( grid + DOWN		  )
					 && self.is_wall( grid + DOWN + RIGHT ) { digable = true },
		};
		digable
	}
}

//End of code.