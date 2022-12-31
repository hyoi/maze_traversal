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
const CHASER_WAIT: f32 = 0.25;
//const CHASER_WAIT: f32 = 0.5;

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
			side: UP,
			wait: Timer::from_seconds( CHASER_WAIT, TimerMode::Once ),
			wandering: Timer::from_seconds( rng.gen_range( CHASER_WAIT_WANDERING ), TimerMode::Once ),
			stop: true,
			lockon: false,
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

		cmds.spawn( SpriteBundle::default() )
			.insert( Sprite { color: CHASER_CALM_COLOR, custom_size, ..default() } )
			.insert( Transform::from_translation( position ).with_rotation( quat ) )
			.insert( Chaser { grid, ..default() } );
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

			//追手が自機を目視できるなら
			if let Some ( ( dxdy1, dxdy2 ) ) = chaser.find( player, &maze, &mut cmds ) //cmdsはデバッグのスプライト表示用
			{	chaser.lockon = true;

				//長辺方向が壁ではないなら
				if ! maze.is_wall( chaser.grid + dxdy1 )
				{	chaser.grid += dxdy1;
					chaser.side = dxdy1;
					chaser.stop = false;
				}
				else if ! maze.is_wall( chaser.grid + dxdy2 )
				{	//短辺方向が壁ではないなら
					chaser.grid += dxdy2;
					chaser.side = dxdy2;
					chaser.stop = false;
				}
				else
				{	//両方向が壁なら、2枚の壁の隙間から目撃したことになる
//					chaser.lockon = false;
				}
			}
			else
			{	//自機を目視できなくても一本道を追跡中なら道なりに追う
				if chaser.lockon && maze.is_passage( chaser.grid )
				{	let back_dxdy = match chaser.side
					{	UP    => DOWN ,
						LEFT  => RIGHT,
						RIGHT => LEFT ,
						DOWN  => UP   ,
					};

					//行き止まりなら背後へ進む
					if maze.is_deadend( chaser.grid )
					{	chaser.grid += back_dxdy;
						chaser.side = back_dxdy;
						chaser.stop = false;
					}
					else
					{	//背後を除く三方で壁の状態を調べる
						let mut next_dxdy = Vec::new();
						for dxdy in FOUR_SIDES
						{	if dxdy != back_dxdy && ! maze.is_wall( chaser.grid + dxdy ) { next_dxdy.push( dxdy ) }
						}

						//一本道なら
						if next_dxdy.len() == 1
						{	chaser.grid += next_dxdy[ 0 ];
							chaser.side = next_dxdy[ 0 ];
							chaser.stop = false;
						}
						else
						{	//交差点なら、乱数で進む方向を決める
							let dxdy = next_dxdy[ rng.gen_range( 0..next_dxdy.len() ) ];
							chaser.grid += dxdy;
							chaser.side = dxdy;
							chaser.stop = false;
//							chaser.lockon = false;
						}
					}
				}
				else
				{	//追跡をやめてゆっくり移動する
					chaser.lockon = false;
					if ! chaser.wandering.tick( time_delta ).finished() { continue }

					//現在の場所が広間なら
					if maze.is_hall( chaser.grid )
					{	//四方でホールのマスを探す
						let mut next_dxdy = Vec::new();
						for dxdy in FOUR_SIDES
						{	if maze.is_hall( chaser.grid + dxdy ) { next_dxdy.push( dxdy ) }
						}

						//ランダムに移動する
						let dxdy = next_dxdy[ rng.gen_range( 0..next_dxdy.len() ) ];
						chaser.grid += dxdy;
						chaser.side = dxdy;
						chaser.stop = false;		//停止フラグを伏せる
						chaser.wandering.reset();	//ウェイトをリセットする
					}
				}
			}

			//ウェイトをリセットする
			chaser.wait.reset();
		}
		else
		{	if chaser.stop { continue }	//停止中なら何もしない

			//スプライトを滑らかに移動させるための中割アニメーション
			let delta = CHASER_MOVE_COEF * time_delta.as_secs_f32();
			let position = &mut transform.translation;
			match chaser.side
			{	UP    => position.y += delta,
				LEFT  => position.x -= delta,
				RIGHT => position.x += delta,
				DOWN  => position.y -= delta,
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

//End of code.