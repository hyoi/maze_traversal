use super::*;

//Pluginの手続き
pub struct PluginPlayer;
impl Plugin for PluginPlayer
{	fn build( &self, app: &mut AppBuilder )
	{	app
		//------------------------------------------------------------------------------------------
		.add_system_set											// ＜GameState::Start＞
		(	SystemSet::on_exit( GameState::Start )				// ＜on_exit()＞
				.with_system( spawn_sprite_player.system() )	// マップ生成後に自機を配置
		)
		//------------------------------------------------------------------------------------------
		.add_system_set											// ＜GameState::Play＞
		(	SystemSet::on_update( GameState::Play )				// ＜on_update()＞
				.with_system( move_sprite_player.system() )		// 自機の移動、ゴール⇒GameState::Clearへ
		)
		//------------------------------------------------------------------------------------------
		.add_system_set											// ＜GameState::Clear＞
		(	SystemSet::on_exit( GameState::Clear )				// ＜on_exit()＞
				.with_system( despawn_sprite_player.system() )	// 自機を削除
		)
		//------------------------------------------------------------------------------------------
		;
	}
}

////////////////////////////////////////////////////////////////////////////////////////////////////

//定義と定数

//移動ウェイト
const PLAYER_WAIT: f32 = 0.09;

//スプライトの動きを滑らかにするための中割係数
const PLAYER_MOVE_COEF: f32 = PIXEL_PER_GRID / PLAYER_WAIT;

//向きを表す列挙型
#[derive(Clone,Copy,PartialEq)]
enum Direction
{	Up,
	Left,
	Right,
	Down,
}

//自機のComponent
struct Player
{	wait: Timer,
	map_location: ( i32, i32 ),
	sprite_location: ( f32, f32 ),
	direction: Direction,
	new_direction: Direction,
	stop: bool,
}

//Sprite
const SPRITE_DEPTH_PLAYER: f32 = 20.0;
const PLAYER_PIXEL: f32   = PIXEL_PER_GRID / 2.5;
const PLAYER_COLOR: Color = Color::YELLOW;

////////////////////////////////////////////////////////////////////////////////////////////////////

//自機のスプライトを初期位置に配置する
fn spawn_sprite_player( maze: Res<GameMap>, mut cmds: Commands )
{	let ( map_x, map_y ) = maze.start_xy;
	let ( sprite_x, sprite_y ) = conv_sprite_coordinates( map_x, map_y );

	let player = Player
	{	wait: Timer::from_seconds( PLAYER_WAIT, false ),
		map_location: ( map_x, map_y ),
		sprite_location: ( sprite_x, sprite_y ),
		direction: Direction::Up,
		new_direction: Direction::Up,
		stop: true,
	};

	//スプライトを初期位置に配置する
	let sprite = sprite_player( player.sprite_location );
	cmds.spawn_bundle( sprite ).insert( player );
}

//自機のスプライトを移動する
fn move_sprite_player
(	mut q: Query<( &mut Player, &mut Transform )>,
	q_visible: Query<&mut Visible>,
	mut maze: ResMut<GameMap>,
	mut state : ResMut<State<GameState>>,
	mut cmds: Commands,
	( time, inkey ): ( Res<Time>, Res<Input<KeyCode>> ),
)
{	let time_delta = time.delta();
	let ( mut player, mut transform ) = q.single_mut().unwrap();

	if ! player.wait.tick( time_delta ).finished()
	{	//停止中なら何も処理しない
		if player.stop { return }

		//スプライトを滑らかに移動させるための中割アニメーション
		let delta = PLAYER_MOVE_COEF * time_delta.as_secs_f32();
		let position = &mut transform.translation;
		match player.direction
		{	Direction::Up    => position.y += delta,
			Direction::Left  => position.x -= delta,
			Direction::Right => position.x += delta,
			Direction::Down  => position.y -= delta,
		}
		player.sprite_location = ( position.x, position.y );

		//スプライト(三角形)の表示向きを更新する
		if player.direction != player.new_direction
		{	let angle = decide_angle( player.direction, player.new_direction );
			let quat = Quat::from_rotation_z( angle.to_radians() );
			transform.rotate( quat );
			player.direction = player.new_direction;
		}
	}
	else
	{	//スプライトの表示位置を更新する
		let ( mut map_x, mut map_y ) = player.map_location;
		let ( sprite_x, sprite_y ) = conv_sprite_coordinates( map_x, map_y );
		let position = &mut transform.translation;
		position.x = sprite_x;
		position.y = sprite_y;
		player.sprite_location = ( position.x, position.y );

		//スプライト(三角形)の表示向きを更新する
		if player.direction != player.new_direction
		{	let angle = decide_angle( player.direction, player.new_direction );
			let quat = Quat::from_rotation_z( angle.to_radians() );
			transform.rotate( quat );
			player.direction = player.new_direction;
		}

		//GOAL判定
		if let MapObj::Goal( opt_dot ) = maze.map[ map_x as usize ][ map_y as usize ]
		{	cmds.entity( opt_dot.unwrap() ).despawn();
//			maze.map[ map_x as usize ][ map_y as usize ] = MapObj::Space;
//			record.score += 1;
		}

		//ゴールしたので、Clearへ遷移する
		if ( map_x, map_y ) == maze.goal_xy
		{	let _ = state.overwrite_set( GameState::Clear );
			return;
		}

		//キー入力を取得する
		let key_left  = inkey.pressed( KeyCode::Left  );
		let key_right = inkey.pressed( KeyCode::Right );
		let key_up    = inkey.pressed( KeyCode::Up    );
		let key_down  = inkey.pressed( KeyCode::Down  );

		//カーソルキーの入力により自機の向きを変える
		if key_left
		{	player.new_direction = Direction::Left;
			player.stop = maze.is_wall_middle_left( map_x, map_y );
			if ! player.stop { map_x -= 1 }
		}
		else if key_right
		{	player.new_direction = Direction::Right;
			player.stop = maze.is_wall_middle_right( map_x, map_y );
			if ! player.stop { map_x += 1 }
		}
		else if key_up
		{	player.new_direction = Direction::Up;
			player.stop = maze.is_wall_upper_center( map_x, map_y );
			if ! player.stop { map_y -= 1 }
		}
		else if key_down
		{	player.new_direction = Direction::Down;
			player.stop = maze.is_wall_lower_center( map_x, map_y );
			if ! player.stop { map_y += 1 }
		}
		else
		{	player.stop = true
		}
		player.map_location = ( map_x, map_y );

		//Dark Modeでプレイヤーの周囲を視覚化する
		maze.show_enclosure_obj( map_x, map_y, q_visible );

		//ウェイトをリセットする
		player.wait.reset();
	}
}

//自機(三角形)の新旧の向きから、表示角度差分を決める
fn decide_angle( old: Direction, new: Direction ) -> f32
{	match old
	{	Direction::Up =>
		{	if matches!( new, Direction::Left  ) { return  90.0 }
			if matches!( new, Direction::Right ) { return -90.0 }
		}
		Direction::Left =>
		{	if matches!( new, Direction::Down  ) { return  90.0 }
			if matches!( new, Direction::Up    ) { return -90.0 }
		}
		Direction::Right =>
		{	if matches!( new, Direction::Up    ) { return  90.0 }
			if matches!( new, Direction::Down  ) { return -90.0 }
		}
		Direction::Down =>
		{	if matches!( new, Direction::Right ) { return  90.0 }
			if matches!( new, Direction::Left  ) { return -90.0 }
		}
	}

	//呼出側でold != newが保証されているので、±90°以外はすべて180°
	180.0
}

//自機のスプライトを削除する
fn despawn_sprite_player( mut q: Query<Entity, With<Player>>, mut cmds: Commands )
{	cmds.entity( q.single_mut().unwrap() ).despawn();
}

////////////////////////////////////////////////////////////////////////////////////////////////////

//自機のスプライトバンドルを生成
fn sprite_player( ( x, y ): ( f32, f32 ) ) -> ShapeBundle
{	GeometryBuilder::build_as
	(	&shapes::RegularPolygon
		{	sides: 3,
			feature: shapes::RegularPolygonFeature::Radius( PLAYER_PIXEL ),
			..shapes::RegularPolygon::default()
		},
		ShapeColors::new( PLAYER_COLOR ),
        DrawMode::Fill( FillOptions::default() ),
		Transform::from_translation( Vec3::new( x, y, SPRITE_DEPTH_PLAYER ) )
	)
}

//End of code.