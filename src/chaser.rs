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
			.with_system( move_sprite_chaser )				// 追手の移動
			.with_system( update_sprite_chasers )			// 追手の回転
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

//追手のスプライトを初期位置に配置する
pub fn spawn_sprite_chasers
(	q: Query<Entity, With<Chaser>>,
	mut maze: ResMut<GameMap>,
//	record: Res<Record>,
	mut cmds: Commands,
)
{	//追手は複数なのでループする
	( 0..=10 ).for_each( | _ |
	{//	let ( grid_x, grid_y ) = CHASER_START_POSITION[ ( record.stage - 1 + i ) % 4 ];
		let ( mut grid_x, mut grid_y );
		loop
		{	grid_x = maze.rng.gen_range( MAP_DIGABLE_X );
			grid_y = maze.rng.gen_range( MAP_DIGABLE_Y );
			if ! maze.is_passageway( grid_x, grid_y ) && ! maze.is_wall( grid_x, grid_y ) { break }
		}
		let ( sprite_x, sprite_y ) = conv_sprite_coordinates( grid_x, grid_y );

		//スプライトを初期位置に配置する
		let chaser = Chaser
		{	map_position: ( grid_x, grid_y ),
			pixel_position: ( sprite_x, sprite_y ),
			pixel_position_old: ( sprite_x, sprite_y ),
			direction: types::Direction::Up,
			wait: Timer::from_seconds( CHASER_WAIT, false ),
			stop: true,
			collision: false,
			speedup: 1.,
		};
		let sprite = sprite_chaser( chaser.pixel_position );
		cmds.spawn_bundle( sprite ).insert( chaser );
	} );
}

//追手のスプライトバンドルを生成
fn sprite_chaser( ( x, y ): ( f32, f32 ) ) -> SpriteBundle
{	let position = Vec3::new( x, y, SPRITE_CHASER_DEPTH );
	let quat = Quat::from_rotation_z( 45_f32.to_radians() ); //45°傾ける

	let color = SPRITE_CHASER_COLOR;
	let custom_size = Some( Vec2::new( SPRITE_CHASER_PIXEL, SPRITE_CHASER_PIXEL ) );

	let sprite = Sprite { color, custom_size, ..Default::default() };
	let transform = Transform::from_translation( position ).with_rotation( quat );

	SpriteBundle { sprite, transform, ..Default::default() }
}

fn move_sprite_chaser
(	mut q_chasers: Query<( &mut Chaser, &mut Transform )>,
	q_player: Query< &Player >,
	time: Res<Time>,
)
{	let time_delta = time.delta().as_secs_f32();
	let player_x;
	let player_y;
	if let Ok ( player ) = q_player.get_single()
	{	player_x = player.map_postion.0;
		player_y = player.map_postion.1;
	}

	q_chasers.for_each_mut
	(	| ( mut chaser, mut transform ) |
		{	if chaser.map_position.0 == player_x && chaser.map_position.1 == player_y
			{	//

			}
		}
	);
}

//追手のスプライトをアニメーションさせる
fn update_sprite_chasers
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