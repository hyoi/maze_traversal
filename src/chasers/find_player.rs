use super::*;

impl Chaser
{	pub fn find( & self, player: &Player, maze: &GameMap, cmds: &mut Commands, ) -> Option<(DxDy,DxDy)>
	//追手から自機が見えるか判定する。
	//追手と自機の間に壁が無いと Some ( dxdy1, dxdy2 )、あれば None を返す。
	//dxdy1とdxdy2は追手が進む方向の候補。（長辺方向と短辺方向）
	//ただし現在の実装には問題があって2枚の壁の隙間ごしに自機が目撃できるので、見える＝移動可能にはならない。
	//隙間からの目撃とは：　追壁
	//　　　　　　　　　　　壁自
	{	let mut x1 = self.grid.x;
		let mut y1 = self.grid.y;
		let mut x2 = player.grid.x;
		let mut y2 = player.grid.y;
		let side_x = ( x1 - x2 ).abs() + 1;
		let side_y = ( y1 - y2 ).abs() + 1;
		let ret_dxdy1;
		let ret_dxdy2;

		//長辺がX方向なら
		if side_x >= side_y
		{	//追手の移動方向の候補（壁の有無は考慮してない）
			ret_dxdy1 = if x1 <= x2 { RIGHT } else { LEFT };
			ret_dxdy2 = if y1 <= y2 { DOWN } else { UP };

			//x1 <= x2 を満たすように位置をswapする
			if x1 > x2
			{	std::mem::swap( &mut x1, &mut x2 );
				std::mem::swap( &mut y1, &mut y2 );
			}

			//pixelの増加量を求める（短辺は＋１または－１ずつ進める）
			let dx = PIXEL_PER_GRID * side_x as f32 / side_y as f32;
			let dy = PIXEL_PER_GRID * if y1 >= y2 { 1.0 } else { -1.0 };

			//ループで使う変数の準備
			let mut grid = MapGrid { x: x1 , y: y1 };
			let mut pixel = grid.into_pixel();
			let mut pool = 0.0;		//長辺方向の切り捨て誤差を蓄える変数
			let mut adjust = 0.0;	//長辺方向の切り捨て誤差がgrid分に達したらループに反映する変数

			//外側が短辺ループで、内側が増加量ずつに分割された長辺ループ
			loop
			{	let mut n = 0;
				loop
				{	let new_px = pixel.x + PIXEL_PER_GRID * n as f32;
					if new_px >= pixel.x + dx + adjust { break } //内側loopの脱出条件

					//壁か？
					grid.x = ( ( new_px - ( PIXEL_PER_GRID - SCREEN_WIDTH ) / 2.0 ) / PIXEL_PER_GRID ) as i32;
					if maze.is_wall( grid ) { return None }	//関数の脱出条件
					n += 1;

					//デバッグ用に視線のスプライトを表示する
					if cfg!( debug_assertions )
					{	let custom_size = Some( Vec2::new( DEBUG_PIXEL, DEBUG_PIXEL ) * 0.2 );
						cmds.spawn( SpriteBundle::default() )
							.insert( Sprite { color: Color::LIME_GREEN, custom_size, ..default() } )
							.insert( Transform::from_translation( Vec3::new( new_px, pixel.y, 15.0 ) ) )
							.insert( DebugSpriteSight );
					}
				}

				//外側loopの脱出条件
				if grid.y == y2 { break }

				//各変数の調整
				grid.y -= dy.signum() as i32;
				pixel += ( dx + adjust, dy );
				let work = ( pixel.x / PIXEL_PER_GRID ).floor() * PIXEL_PER_GRID;
				pool += pixel.x - work;
				pixel.x = work;			//次の開始位置
				adjust = ( pool / PIXEL_PER_GRID ).floor() * PIXEL_PER_GRID;
				pool -= adjust;
			}
		}
		else
		{	//追手の移動方向の候補（壁の有無は考慮してない）
			ret_dxdy1 = if y1 <= y2 { DOWN } else { UP };
			ret_dxdy2 = if x1 <= x2 { RIGHT } else { LEFT };

			//長辺がY方向なので y1 <= y2 を満たすように位置をswapする
			if y1 > y2
			{	std::mem::swap( &mut x1, &mut x2 );
				std::mem::swap( &mut y1, &mut y2 );
			}

			//pixelの増加量を求める（短辺は＋１または－１ずつ進める）
			let dx = PIXEL_PER_GRID * if x1 <= x2 { 1.0 } else { -1.0 };
			let dy = PIXEL_PER_GRID * side_y as f32 / side_x as f32;

			//ループで使う変数の準備
			let mut grid = MapGrid { x: x1, y: y1 };
			let mut pixel = grid.into_pixel();
			let mut pool = 0.0;		//長辺方向の切り捨て誤差を蓄える変数
			let mut adjust = 0.0;	//長辺方向の切り捨て誤差がgrid分に達したらループに反映する変数

			//外側が短辺ループで、内側が増加量ずつに分割された長辺ループ
			loop
			{	let mut n = 0;
				loop
				{	let new_py = pixel.y - PIXEL_PER_GRID * n as f32;
					if new_py <= pixel.y - dy - adjust { break } //内側loopの脱出条件

					//壁か？
					grid.y = ( ( ( SCREEN_HEIGHT - PIXEL_PER_GRID ) / 2.0 - new_py ) / PIXEL_PER_GRID ) as i32 - 1;
					if maze.is_wall( grid ) { return None }	//関数の脱出条件
					n += 1;

					//デバッグ用に視線のスプライトを表示する
					if cfg!( debug_assertions )
					{	let custom_size = Some( Vec2::new( DEBUG_PIXEL, DEBUG_PIXEL ) * 0.2 );
						 cmds.spawn( SpriteBundle::default() )
							.insert( Sprite { color: Color::CYAN, custom_size, ..default() } )
							.insert( Transform::from_translation( Vec3::new( pixel.x, new_py, 15.0 ) ) )
							.insert( DebugSpriteSight );
					}
				}

				//外側loopの脱出条件
				if grid.x == x2 { break }

				//各変数の調整
				grid.x += dx.signum() as i32;
				pixel += ( dx, - ( dy + adjust ) );
				let work = ( pixel.y / PIXEL_PER_GRID ).ceil() * PIXEL_PER_GRID;
				pool += work - pixel.y;
				pixel.y = work;			//次の開始位置
				adjust = ( pool / PIXEL_PER_GRID ).floor() * PIXEL_PER_GRID;
				pool -= adjust;
			}
		}

		Some ( ( ret_dxdy1, ret_dxdy2 ) )	//目視できた
	}
}

//End of code.