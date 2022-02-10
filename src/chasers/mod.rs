use super::*;

//internal modules
mod find_player;

//Pluginの手続き
pub struct PluginChaser;
impl Plugin for PluginChaser
{	fn build( &self, app: &mut App )
	{	app
		//------------------------------------------------------------------------------------------
		.add_system_set												// ＜GameState::Start＞
		(	SystemSet::on_exit( GameState::Start )					// ＜on_exit()＞
				.with_system( spawn_sprite_chasers )				// マップ生成後に追手を配置
		)
		//==========================================================================================
		.add_system_set												// ＜GameState::Play＞
		(	SystemSet::on_update( GameState::Play )					// ＜on_update()＞
				.label( "DebugSpriteSight" )
				.with_system( despawn_entity::<DebugSpriteSight> )
		)
		.add_system_set												// ＜GameState::Play＞
		(	SystemSet::on_update( GameState::Play )					// ＜on_update()＞
				.after( "DebugSpriteSight" )
				.with_system( move_sprite_chasers )					// 追手の移動
				.with_system( rotate_sprite_chasers )				// 追手の回転
		)
		//==========================================================================================
		.add_system_set												// ＜GameState::Clear＞
		(	SystemSet::on_exit( GameState::Clear )					// ＜on_exit()＞
				.with_system( despawn_entity::<Chaser> )			// 追手を削除
		)
		//==========================================================================================
		.add_system_set												// ＜GameState::Over＞
		(	SystemSet::on_enter( GameState::Over )					// ＜on_enter()＞
				.with_system( show_ui::<MessageOver> )				// GameOverメッセージを表示する
		)
		.add_system_set												// ＜GameState::Over＞
		(	SystemSet::on_update( GameState::Over )					// ＜on_update()＞
				.with_system( countdown_to_start::<MessageOver> )	// CD完了⇒GameState::Startへ
		)
		.add_system_set												// ＜GameState::Over＞
		(	SystemSet::on_exit( GameState::Over )					// ＜on_exit()＞
				.with_system( despawn_entity::<Player> )			// 自機を削除
				.with_system( despawn_entity::<Chaser> )			// 追手を削除
				.with_system( hide_ui::<MessageOver> )				// GAmeOverメッセージを隠す
				.with_system( init_record )							// 初期化
		)
		//------------------------------------------------------------------------------------------
		;
	}
}

////////////////////////////////////////////////////////////////////////////////////////////////////

//Sprite
const CHASER_PIXEL: f32 = PIXEL_PER_GRID / 2.0;
const CHASER_CALM_COLOR: Color = Color::GREEN;
const CHASER_EXCITE_COLOR: Color = Color::RED;

//移動ウェイト
//const CHASER_WAIT: f32 = 0.09;
const CHASER_WAIT: f32 = 0.5;

//うろうろする際のゆっくり移動ウェイト
use std::ops::Range;
const CHASER_WAIT_WANDERING: Range<f32> = 0.5..3.5;

//スプライトの動きを滑らかにするための中割係数
const CHASER_MOVE_COEF  : f32 = PIXEL_PER_GRID / CHASER_WAIT;

//Default
impl Default for Chaser
{	fn default() -> Self
	{	let mut rng = rand::thread_rng();
		Self
		{	grid: MapGrid::default(),
			pixel: Pixel::default(),
			// pixel_old: Pixel::default(),
			side: FourSides::Up,
			wait: Timer::from_seconds( CHASER_WAIT, false ),
			wandering: Timer::from_seconds( rng.gen_range( CHASER_WAIT_WANDERING ), false ),
			stop: true,
			lockon: false,
			// collision: false,
			// speedup: 1.0,
		}
	}
}

#[derive(Component)]
struct DebugSpriteSight;
const DEBUG_PIXEL: f32 = PIXEL_PER_GRID;

////////////////////////////////////////////////////////////////////////////////////////////////////

//追手のスプライトを広間に配置する
fn spawn_sprite_chasers
(	mut maze: ResMut<GameMap>,
	mut cmds: Commands,
)
{	//追手は複数なのでループする
	for _ in 0..( maze.halls() / 50 )
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
			.insert( Sprite { color: CHASER_CALM_COLOR, custom_size, ..Default::default() } )
			.insert( Transform::from_translation( position ).with_rotation( quat ) )
			.insert( Chaser { grid, pixel, ..Default::default() } );
	};
}

//追手のスプライトを移動する
fn move_sprite_chasers
(	mut q_chasers: Query<( &mut Chaser, &mut Transform )>,
	q_player: Query< &Player >,
	maze: Res<GameMap>,
	mut record: ResMut<Record>,
	mut state: ResMut<State<GameState>>,
	time: Res<Time>,
	mut cmds: Commands,
)
{	let time_delta = time.delta();
	let mut rng = rand::thread_rng();
	let player = q_player.get_single().unwrap();

	//追手は複数なのでQuery結果をループして処理する
	for ( mut chaser, mut transform ) in q_chasers.iter_mut()
	{	//自機と重なったらHPを減らし、ゼロになったらOverへ（暫定処理）
		if chaser.grid == player.grid
		{	record.hp -= 1.0;
			if record.hp <= 0.0
			{	let _ = state.overwrite_set( GameState::Over );
				return;
			}
		}

		if chaser.wait.tick( time_delta ).finished()
		{	//スプライトの表示位置をグリッドに合わせて更新する
			let pixel = chaser.grid.into_pixel();

			let position = &mut transform.translation;
			position.x = pixel.x;
			position.y = pixel.y;
			chaser.stop = true;		//一旦 停止フラグを立てる

			/* 次の移動の決め方
			 * ・視線が通っていれば追跡フラグを立て、でなければ伏せる。
			 * 　- ただし、現在追跡中で通路が一本道なら(非交差点)、視線が切れても追跡フラグを立てたままにする。
			 * ・追跡フラグが立っている場合
			 * 　- 移動速度を、自機と同じにする。
			 * 　- 広間なら、自機に対してxとyを寄せる（斜め移動なし）
			 * 　- 通路なら、道なりに進む。
			 * 　　　交差点では、自機が見えていればそっちへ移動、見えていなければ追跡フラグを伏せる。
			 * 　- 自機とすれ違ったら、自機を追う方へ方向を変える。
			 * ・追跡フラグが伏している場合
			 * 　- 移動速度を、ゆっくりにする。
			 * 　- 広間なら、らうろうろする。広間からは出ない。
			 * 　- 通路なら、道なりに進む。
			 * 　　　他の道との交差点ではランダムに曲がる。行き止まりはバックする。広間に入るまで続ける。
			 */

			//現在追跡中で通路が一本道なら、追跡フラグを立てたままにする
			chaser.lockon = if chaser.lockon && maze.is_passage( chaser.grid ) 
			{	//四方の壁の状態を取得する
				let mut count = 0;
				FOUR_SIDES.iter()
					.for_each( | dxdy | if ! maze.is_wall( chaser.grid + dxdy ) { count += 1 } );

				if count <= 2 { true } else { false }	//一本道 else 交差点
			}
			else
			{	//そうでないなら、視線が通っているかチェックした結果に従う
				! maze.is_wall_blocking_sight( chaser.grid, player.grid, &mut cmds )				
			};

			if chaser.lockon
			{	//現在の場所が広間なら
				if maze.is_hall( chaser.grid )
				{	let x1 = chaser.grid.x as i32;
					let y1 = chaser.grid.y as i32;
					let x2 = player.grid.x as i32;
					let y2 = player.grid.y as i32;
					let mut next = Vec::new();

					//X方向
					let ( dxdy, side ) = if x1 < x2 { ( RIGHT, FourSides::Right ) } else { ( LEFT, FourSides::Left ) };
					if ! maze.is_wall( chaser.grid + dxdy ) { next.push( ( dxdy, side ) ) }

					//Y方向
					let ( dxdy, side ) = if y1 < y2 { ( DOWN, FourSides::Down ) } else { ( UP, FourSides::Up ) };
					if ! maze.is_wall( chaser.grid + dxdy ) { next.push( ( dxdy, side ) ) }

					//XもYも壁なら
					if next.is_empty() { continue }

					let x = rng.gen_range( 0..next.len() );
					chaser.grid += next[ x ].0;
					chaser.side = next[ x ].1;
					chaser.stop = false;		//停止フラグを伏せる
					chaser.wait.reset();		//ウェイトをリセットする
					chaser.wandering.reset();	//ウェイトをリセットする
				}
			}
			else
			{	//移動速度をゆっくりにする
				if ! chaser.wandering.tick( time_delta ).finished() { continue }

				//現在の場所が広間なら
				if maze.is_hall( chaser.grid )
				{	//四方でホールのマスを探す
					let mut next = Vec::new();
					for dxdy in FOUR_SIDES
					{	let next_grid = chaser.grid + dxdy;
						if maze.is_hall( next_grid )
						{	if matches!( dxdy, UP    ) { next.push( ( next_grid, FourSides::Up    ) ) }
							if matches!( dxdy, LEFT  ) { next.push( ( next_grid, FourSides::Left  ) ) }
							if matches!( dxdy, RIGHT ) { next.push( ( next_grid, FourSides::Right ) ) }
							if matches!( dxdy, DOWN  ) { next.push( ( next_grid, FourSides::Down  ) ) }
						}
					}

					//ランダムに移動する
					let x = rng.gen_range( 0..next.len() );
					chaser.grid = next[ x ].0;
					chaser.side = next[ x ].1;
					chaser.stop = false;		//停止フラグを伏せる
					chaser.wait.reset();		//ウェイトをリセットする
					chaser.wandering.reset();	//ウェイトをリセットする
				}
			}
		}
		else
		{	if chaser.stop { continue }	//停止中なら何もしない

			//スプライトを滑らかに移動させるための中割アニメーション
			let delta = CHASER_MOVE_COEF * time_delta.as_secs_f32();
			let position = &mut transform.translation;
			match chaser.side
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
(	mut q: Query<( &Chaser, &mut Transform, &mut Sprite )>,
	time: Res<Time>,
)
{	let time_delta = time.delta().as_secs_f32();
	let angle = 360.0 * time_delta;
	let quat = Quat::from_rotation_z( angle.to_radians() );

	q.for_each_mut
	(	| ( chaser, mut transform, mut sprite ) |
		{	transform.rotate( quat );	//回転させる
			let color = if chaser.lockon { CHASER_EXCITE_COLOR } else { CHASER_CALM_COLOR };
			sprite.color = color;	//色を変える
		}
	);
}

////////////////////////////////////////////////////////////////////////////////////////////////////

//GameOverのon_exit()でRecordを初期化する
fn init_record( mut record: ResMut<Record> ) { *record = Record::default(); }

//End of code.