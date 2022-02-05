use super::*;

//Pluginの手続き
pub struct PluginChaser;
impl Plugin for PluginChaser
{	fn build( &self, app: &mut App )
	{	app
		//------------------------------------------------------------------------------------------
		.add_system_set										// ＜GameState::Start＞
		(	SystemSet::on_exit( GameState::Start )			// ＜on_exit()＞
				.with_system( spawn_sprite_chasers )		// マップ生成後に追手を配置
		)
		//------------------------------------------------------------------------------------------
		.add_system_set										// ＜GameState::Play＞
		(	SystemSet::on_update( GameState::Play )			// ＜on_update()＞
			.with_system( move_sprite_chasers )				// 追手の移動
			.with_system( rotate_sprite_chasers )			// 追手の回転
		)
		//------------------------------------------------------------------------------------------
		.add_system_set										// ＜GameState::Clear＞
		(	SystemSet::on_exit( GameState::Clear )			// ＜on_exit()＞
				.with_system( despawn_entity::<Chaser> )	// 追手を削除
		)
		//------------------------------------------------------------------------------------------
		;
	}
}

////////////////////////////////////////////////////////////////////////////////////////////////////

//Sprite
const CHASER_PIXEL: f32 = PIXEL_PER_GRID / 2.0;
const CHASER_COLOR: Color = Color::RED;

//移動ウェイト
const CHASER_WAIT   : f32 = 0.5;
//const CHASER_ACCEL: f32 = 0.4; //スピードアップの割増

//スプライトの動きを滑らかにするための中割係数
const CHASER_MOVE_COEF  : f32 = PIXEL_PER_GRID / CHASER_WAIT;
//const CHASER_ROTATE_COEF: f32 = 90. / CHASER_WAIT;

//Default
impl Default for Chaser
{	fn default() -> Self
	{	let mut rng = rand::thread_rng();
		Self
		{	map_xy: MapGrid::default(),
			pixel_xy: Pixel::default(),
			pixel_xy_old: Pixel::default(),
			direction: FourSides::Up,
			wait: Timer::from_seconds( CHASER_WAIT, false ),
			wandering: Timer::from_seconds( rng.gen_range( 0.5..3.5 ), false ),
			stop: true,
			collision: false,
			speedup: 1.0,
		}
	}
}

////////////////////////////////////////////////////////////////////////////////////////////////////

//追手のスプライトを広間に配置する
fn spawn_sprite_chasers
(	mut maze: ResMut<GameMap>,
	mut cmds: Commands,
)
{	//追手は複数なのでループする
	( 0..=10 ).for_each( | _ |		//取り敢えず10個固定
	{	let mut grid = MapGrid::default();
		loop
		{	grid.x = maze.rng().gen_range( RANGE_MAP_INNER_X );
			grid.y = maze.rng().gen_range( RANGE_MAP_INNER_Y );
			if maze.is_hall( grid ) { break }
		}
		let pixel = grid.into_pixel();

		//スプライトを初期位置に配置する
		let position = Vec3::new( pixel.x, pixel.y, SPRITE_DEPTH_CHASER );
		let quat = Quat::from_rotation_z( 45_f32.to_radians() ); //45°傾ける
		let custom_size = Some( Vec2::new( CHASER_PIXEL, CHASER_PIXEL ) );

		cmds.spawn_bundle( SpriteBundle::default() )
			.insert( Sprite { color: CHASER_COLOR, custom_size, ..Default::default() } )
			.insert( Transform::from_translation( position ).with_rotation( quat ) )
			.insert( Chaser { map_xy: grid, pixel_xy: pixel, ..Default::default() } );
	} );
}

//追手のスプライトを移動する
fn move_sprite_chasers
(	mut q_chasers: Query<( &mut Chaser, &mut Transform )>,
	q_player: Query< &Player >,
	maze: Res<GameMap>,
	mut state: ResMut<State<GameState>>,
	time: Res<Time>,
)
{	let time_delta = time.delta();
	let mut rng = rand::thread_rng();
	let player = q_player.get_single().unwrap();

	//追手は複数なのでQuery結果をループして処理する
	for ( mut chaser, mut transform ) in q_chasers.iter_mut()
	{	if chaser.wait.tick( time_delta ).finished()
		{	//スプライトの表示位置をグリッドに合わせて更新する
			let grid = chaser.map_xy;
			let pixel = grid.into_pixel();

			let position = &mut transform.translation;
			position.x = pixel.x;
			position.y = pixel.y;
			chaser.stop = true;		//一旦 停止フラグを立てる

			//自機と重なったらOverへ（暫定処理）
			if grid == player.map_xy
			{	let _ = state.overwrite_set( GameState::Over );
				return;
			}

			//次の移動
			//・追跡モード(視線が切れていない場合Playerの方へ。広間なら斜め移動あり)
			//・通路モード(道なりに進み袋小路なら折り返す。分かれ道では適当に曲がる)
			//・広間モード(ワンダリングするが広間から出ない)

			//視線が通っているか？
			let mut flag_chase = false;
			if grid.x == player.map_xy.x || grid.y == player.map_xy.y
			{	let px = player.map_xy.x;
				let py = player.map_xy.y;
				let range_x = if grid.x < px { grid.x..px } else { px..grid.x };
				let range_y = if grid.y < py { grid.y..py } else { py..grid.y };
				let mut flag_x = false;
				let mut flag_y = false;
				range_x.for_each( | x | if maze.is_wall( MapGrid { x, y: grid.y } ) { flag_x = true } );
				range_y.for_each( | y | if maze.is_wall( MapGrid { x: grid.x, y } ) { flag_y = true } );

				flag_chase = ! flag_x && ! flag_y;
			}

			if flag_chase
			{	//追跡モード
				dbg!(1);
				chaser.wait.reset();	//ウェイトをリセットする
			}
			else if maze.is_passage( grid )
			{	//通路モード
				dbg!(2);
				chaser.wait.reset();	//ウェイトをリセットする
			}
			else
			{	//広間モード
				if ! chaser.wandering.tick( time_delta ).finished() { continue }

				//四方でホールのマスを探す
				let mut four_sides = Vec::new();
				for dxdy in FOUR_SIDES
				{	let next = grid + dxdy;
					if maze.is_hall( next )
					{	if matches!( dxdy, UP    ) { four_sides.push( ( next, FourSides::Up    ) ) }
						if matches!( dxdy, LEFT  ) { four_sides.push( ( next, FourSides::Left  ) ) }
						if matches!( dxdy, RIGHT ) { four_sides.push( ( next, FourSides::Right ) ) }
						if matches!( dxdy, DOWN  ) { four_sides.push( ( next, FourSides::Down  ) ) }
					}
				}

				//ランダムに移動する
				let x = rng.gen_range( 0..four_sides.len() );
				chaser.map_xy    = four_sides[ x ].0;
				chaser.direction = four_sides[ x ].1;
				chaser.stop = false;		//停止フラグを倒す
				chaser.wait.reset();		//ウェイトをリセットする
				chaser.wandering.reset();	//ウェイトをリセットする
			}
		}
		else
		{	if chaser.stop { continue }	//停止中なら何もしない

			//スプライトを滑らかに移動させるための中割アニメーション
			let delta = CHASER_MOVE_COEF * time_delta.as_secs_f32();
			let position = &mut transform.translation;
			match chaser.direction
			{	FourSides::Up    => position.y += delta,
				FourSides::Left  => position.x -= delta,
				FourSides::Right => position.x += delta,
				FourSides::Down  => position.y -= delta,
			}
		}
	}
}

//追手のスプライトをアニメーションさせる
fn rotate_sprite_chasers
(	mut q: Query< &mut Transform, With<Chaser>>,
	time: Res<Time>,
)
{	let time_delta = time.delta().as_secs_f32();
	let angle = 360.0 * time_delta;
	let quat = Quat::from_rotation_z( angle.to_radians() );

	//回転させる
	q.for_each_mut( | mut transform | transform.rotate( quat ) );
}

//End of code.