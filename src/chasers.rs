use super::*;

//external modules
use rand::prelude::*;

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
	{	Self
		{	map_xy: MapGrid::default(),
			pixel_xy: Pixel::default(),
			pixel_xy_old: Pixel::default(),
			direction: FourSides::Up,
			wait: Timer::from_seconds( CHASER_WAIT, false ),
			stop: true,
			collision: false,
			speedup: 1.0,
		}
	}
}

////////////////////////////////////////////////////////////////////////////////////////////////////

//追手のスプライトを広間に配置する
pub fn spawn_sprite_chasers
(	mut maze: ResMut<GameMap>,
	mut cmds: Commands,
)
{	//追手は複数なのでループする
	( 0..=10 ).for_each( | _ |
	{	let ( mut x, mut y );
		loop
		{	x = maze.rng.gen_range( RANGE_MAP_INNER_X );
			y = maze.rng.gen_range( RANGE_MAP_INNER_Y );
			if maze.is_hall( x, y ) { break }
		}
		let map_xy   = MapGrid { x, y };
		let pixel_xy = map_xy.into_pixel();

		//スプライトを初期位置に配置する
		let position = Vec3::new( pixel_xy.x, pixel_xy.y, SPRITE_DEPTH_CHASER );
		let quat = Quat::from_rotation_z( 45_f32.to_radians() ); //45°傾ける
		let custom_size = Some( Vec2::new( CHASER_PIXEL, CHASER_PIXEL ) );

		cmds.spawn_bundle( SpriteBundle::default() )
			.insert( Sprite { color: CHASER_COLOR, custom_size, ..Default::default() } )
			.insert( Transform::from_translation( position ).with_rotation( quat ) )
			.insert( Chaser { map_xy, pixel_xy, ..Default::default() } );
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
			let map = chaser.map_xy;
			let pixel = map.into_pixel();
			let position = &mut transform.translation;
			position.x = pixel.x;
			position.y = pixel.y;

			//自機と重なったらOverへ
			if map.x == player.map_xy.x && map.y == player.map_xy.y
			{	let _ = state.overwrite_set( GameState::Over );
				return;
			}

			//次の移動
			//・追跡モード(視線が切れていない場合Playerの方へ。広間なら斜め移動あり)
			//・通路モード(道なりに進み袋小路なら折り返す。分かれ道では適当に曲がる)
			//・広間モード(ワンダリングするが広間から出ない)

			//視線が通っているか？
			let mut flag_chase = false;
			if map.x == player.map_xy.x || map.y == player.map_xy.y
			{	let cx = map.x;
				let cy = map.y;
				let px = player.map_xy.x;
				let py = player.map_xy.y;
				let range_x = if cx < px { cx..px } else { px..cx };
				let range_y = if cy < py { cy..py } else { py..cy };
				let mut flag_x = false;
				let mut flag_y = false;
				range_x.for_each( | x | if maze.is_wall( x, cy ) { flag_x = true } );
				range_y.for_each( | y | if maze.is_wall( cx, y ) { flag_y = true } );

				flag_chase = ! flag_x && ! flag_y;
			}

			if flag_chase
			{	//追跡モード
			}
			else if maze.is_passageway( map.x, map.y )
			{	//通路モード
			}
			else
			{	//広間モード
				let mut four_sides = Vec::new();
				for ( dx, dy ) in FOUR_SIDES
				{	let MapGrid { mut x, mut y } = map;
					x += dx - 1;
					y += dy - 1;
					if maze.is_hall( x, y )
					{	if matches!( ( dx, dy ), UP    ) { four_sides.push( ( MapGrid { x, y }, FourSides::Up    ) ) }
						if matches!( ( dx, dy ), LEFT  ) { four_sides.push( ( MapGrid { x, y }, FourSides::Left  ) ) }
						if matches!( ( dx, dy ), RIGHT ) { four_sides.push( ( MapGrid { x, y }, FourSides::Right ) ) }
						if matches!( ( dx, dy ), DOWN  ) { four_sides.push( ( MapGrid { x, y }, FourSides::Down  ) ) }
					}
				}
				let x = rng.gen_range( 0..four_sides.len() );
				chaser.map_xy = four_sides[ x ].0;
				chaser.direction = four_sides[ x ].1;
			}

			//ウェイトをリセットする
			chaser.wait.reset();
		}
		else
		{	//スプライトを滑らかに移動させるための中割アニメーション
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