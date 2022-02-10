use super::*;

impl GameMap
{	//壁がグリッド間の視界を遮っているか判定する
	pub fn is_wall_blocking_sight( & self, grid1: MapGrid, grid2: MapGrid, cmds: &mut Commands, ) -> bool
	{	let mut x1 = grid1.x as i32;
		let mut y1 = grid1.y as i32;
		let mut x2 = grid2.x as i32;
		let mut y2 = grid2.y as i32;
		let side_x = ( x1 - x2 ).abs() + 1;
		let side_y = ( y1 - y2 ).abs() + 1;

		//長辺X方向、短辺Y方向なら
		if side_x >= side_y
		{	//長辺がX方向なので x1 <= x2 を満たすようにgridをswapする
			if x1 > x2
			{	std::mem::swap( &mut x1, &mut x2 );
				std::mem::swap( &mut y1, &mut y2 );
			}

			//pixelの増加量を求める（短辺は＋１または－１ずつ変化させる）
			let dx = PIXEL_PER_GRID * side_x as f32 / side_y as f32;
			let dy = PIXEL_PER_GRID * if y1 >= y2 { 1.0 } else { -1.0 };

			//ループで使う変数の準備
			let mut grid = MapGrid { x: x1 as usize, y: y1 as usize };
			let mut pixel = grid.into_pixel();
			let mut rem = 0.0;		//長辺方向の切り捨て誤差を蓄える変数
			let mut adjust = 0.0;	//長辺方向の切り捨て誤差がgrid分に達したらループに反映する変数

			//外側が短辺ループで、内側が増加量ずつに分割された長辺ループ
			loop
			{	let mut n = 0;
				loop
				{	let new_px = pixel.x + PIXEL_PER_GRID * n as f32;
					if new_px >= pixel.x + dx + adjust { break } //内側loopの脱出条件

					grid.x = ( ( new_px - ( PIXEL_PER_GRID - SCREEN_WIDTH ) / 2.0 ) / PIXEL_PER_GRID ) as usize;
					if self.is_wall( grid ) { return true }	//関数の脱出条件
					n += 1;

					//デバッグ用に視線のスプライトを表示する
					if cfg!( debug_assertions )
					{	let custom_size = Some( Vec2::new( DEBUG_PIXEL, DEBUG_PIXEL ) * 0.2 );
						cmds.spawn_bundle( SpriteBundle::default() )
							.insert( Sprite { color: Color::LIME_GREEN, custom_size, ..Default::default() } )
							.insert( Transform::from_translation( Vec3::new( new_px, pixel.y, 15.0 ) ) )
							.insert( DebugSpriteSight );
					}
				}

				//外側loopの脱出条件
				if grid.y as i32 == y2 { break }
				grid.y = ( grid.y as i32 - dy.signum() as i32 ) as usize;

				pixel += ( dx + adjust, dy );
				let work = ( pixel.x / PIXEL_PER_GRID ).floor() * PIXEL_PER_GRID;
				rem += pixel.x - work;
				pixel.x = work;			//次の開始位置
				adjust = ( rem / PIXEL_PER_GRID ).floor() * PIXEL_PER_GRID;
				rem -= adjust;
			}
		}
		else //長辺Y方向、短辺X方向
		{	//長辺がY方向なので y1 <= y2 を満たすようにgridをswapする
			if y1 > y2
			{	std::mem::swap( &mut x1, &mut x2 );
				std::mem::swap( &mut y1, &mut y2 );
			}

			//pixelの増加量を求める（短辺は＋１または－１ずつ変化させる）
			let dx = PIXEL_PER_GRID * if x1 <= x2 { 1.0 } else { -1.0 };
			let dy = PIXEL_PER_GRID * side_y as f32 / side_x as f32;

			//ループで使う変数の準備
			let mut grid = MapGrid { x: x1 as usize, y: y1 as usize };
			let mut pixel = grid.into_pixel();
			let mut rem = 0.0;		//長辺方向の切り捨て誤差を蓄える変数
			let mut adjust = 0.0;	//長辺方向の切り捨て誤差がgrid分に達したらループに反映する変数

			//外側が短辺ループで、内側が増加量ずつに分割された長辺ループ
			loop
			{	let mut n = 0;
				loop
				{	let new_py = pixel.y - PIXEL_PER_GRID * n as f32;
					if new_py <= pixel.y - dy - adjust { break } //内側loopの脱出条件

					grid.y = ( ( ( SCREEN_HEIGHT - PIXEL_PER_GRID ) / 2.0 - new_py ) / PIXEL_PER_GRID - 1.0 ) as usize;
					if self.is_wall( grid ) { return true }	//関数の脱出条件
					n += 1;

					//デバッグ用に視線のスプライトを表示する
					if cfg!( debug_assertions )
					{	let custom_size = Some( Vec2::new( DEBUG_PIXEL, DEBUG_PIXEL ) * 0.2 );
						 cmds.spawn_bundle( SpriteBundle::default() )
							.insert( Sprite { color: Color::CYAN, custom_size, ..Default::default() } )
							.insert( Transform::from_translation( Vec3::new( pixel.x, new_py, 15.0 ) ) )
							.insert( DebugSpriteSight );
					}
				}

				//外側loopの脱出条件
				if grid.x as i32 == x2 { break }
				grid.x = ( grid.x as i32 + dx.signum() as i32 ) as usize;

				pixel += ( dx, - ( dy + adjust ) );
				let work = ( pixel.y / PIXEL_PER_GRID ).ceil() * PIXEL_PER_GRID;
				rem += work - pixel.y;
				pixel.y = work;			//次の開始位置
				adjust = ( rem / PIXEL_PER_GRID ).floor() * PIXEL_PER_GRID;
				rem -= adjust;
			}
		}

		false	//壁は視線を遮っていない
	}
}

//End of code.