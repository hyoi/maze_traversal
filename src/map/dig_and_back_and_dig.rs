use super::*;

//二型迷路：ランダムに掘り進み、行き止まりは後戻りして掘れる場所を探す。掘り尽くすまで掘りまくる
pub fn dig_and_back_and_dig( maze: &mut GameStage )
{	let mut map_xy = maze.start_xy;
	map_xy.1 -= 1; //maze.start_xyの直上(y-1)がトンネル掘りの開始座標

	//トンネルを掘る
	let mut digable_walls = Vec::new();
	let mut backtrack;
	loop
	{	//上下左右にある掘削候補と戻り路を記録する
		digable_walls.clear();
		backtrack = ( 0, 0 );
		for ( dx, dy ) in DIRECTION.iter()
		{	let tmp_x = map_xy.0 + dx;
			let tmp_y = map_xy.1 + dy;
			let tmp_xy = ( tmp_x, tmp_y ); 

			//外壁は掘れない
			if ! DIGABLE_X.contains( &tmp_x )
			|| ! DIGABLE_Y.contains( &tmp_y ) { continue }
	
			//上下左右の座標のオブジェクトを調べる
			match maze.map[ tmp_x as usize ][ tmp_y as usize ]
			{	MapObj::Dot1(_)
					=> backtrack = tmp_xy,
				MapObj::Wall(_) if is_digable_wall( maze, tmp_xy, ( *dx, *dy ) )
					=> digable_walls.push( tmp_xy ),
				_	=> {}
			}
		}

		//掘れる壁が見つからないなら迷路完成
		if digable_walls.is_empty() && backtrack == ( 0, 0 ) { break }

		if ! digable_walls.is_empty()
		{	//掘削する方向をランダムに決めて、掘る
			map_xy = digable_walls[ maze.rng.gen_range( 0..digable_walls.len() ) ];
			maze.map[ map_xy.0 as usize ][ map_xy.1 as usize ] = MapObj::Dot1( None );
		}
		else
		{	//掘れる壁がないので現在位置に行き止まり情報「dot2」を書き込み、後戻りする
			maze.map[ map_xy.0 as usize ][ map_xy.1 as usize ] = MapObj::Dot2( None );
			map_xy = backtrack;
		}
	}

	//三型迷路の作成関数を流用して、道幅拡張工事
	find_and_destroy_digable_walls( maze );
} 

//進行方向の壁が掘れるか調べる
fn is_digable_wall( maze: &GameStage, ( x, y ): ( i32, i32 ), direction: ( i32, i32 ) ) -> bool
{	let objs = maze.enclosure( x, y );
	match direction
	{	UP    if matches!( objs.upper_left  , MapObj::Wall(_) )
			  && matches!( objs.upper_center, MapObj::Wall(_) )	// 壁壁壁
			  && matches!( objs.upper_right , MapObj::Wall(_) )	// 壁Ｘ壁
			  && matches!( objs.middle_left , MapObj::Wall(_) )
			  && matches!( objs.middle_right, MapObj::Wall(_) ) => return true,
		LEFT  if matches!( objs.upper_left  , MapObj::Wall(_) )	// 壁壁
			  && matches!( objs.upper_center, MapObj::Wall(_) )	// 壁Ｘ
			  && matches!( objs.middle_left , MapObj::Wall(_) )	// 壁壁
			  && matches!( objs.lower_left  , MapObj::Wall(_) )
			  && matches!( objs.lower_center, MapObj::Wall(_) ) => return true,
		RIGHT if matches!( objs.upper_center, MapObj::Wall(_) )	// 壁壁
			  && matches!( objs.upper_right , MapObj::Wall(_) )	// Ｘ壁
			  && matches!( objs.middle_right, MapObj::Wall(_) )	// 壁壁
			  && matches!( objs.lower_center, MapObj::Wall(_) )
			  && matches!( objs.lower_right , MapObj::Wall(_) ) => return true,
		DOWN  if matches!( objs.middle_left , MapObj::Wall(_) )
			  && matches!( objs.middle_right, MapObj::Wall(_) )	// 壁Ｘ壁
			  && matches!( objs.lower_left  , MapObj::Wall(_) )	// 壁壁壁
			  && matches!( objs.lower_center, MapObj::Wall(_) )
			  && matches!( objs.lower_right , MapObj::Wall(_) ) => return true,
		_ => {}
	}

	false
}

//End of code.