use super::*;

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
pub struct Player
{	wait: Timer,
	map_location: ( usize, usize ),
	sprite_location: ( f32, f32 ),
	direction: Direction,
	new_direction: Direction,
	stop: bool,
}

////////////////////////////////////////////////////////////////////////////////

//自機のスプライトを初期位置に配置する
pub fn spawn_sprite_player( stage : Res<GameStage>, mut cmds: Commands )
{	let ( map_x, map_y ) = stage.start_xy;
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

////////////////////////////////////////////////////////////////////////////////

//自機のスプライトを移動する
pub fn move_sprite_player
(	mut q_player: Query<( &mut Player, &mut Transform )>,
	( mut stage, mut record ): ( ResMut<GameStage>, ResMut<GameRecord> ),
	mut event: EventWriter<GameState>,
	mut cmds: Commands,
	( time, inkey ): ( Res<Time>, Res<Input<KeyCode>> ),
)
{	let time_delta = time.delta();
	let ( mut player, mut transform ) = q_player.single_mut().unwrap();

	if ! player.wait.tick( time_delta ).finished()
	{	//停止中なら何も処理しない
		if player.stop { return }

		//スプライトを滑らかに移動させるための中割アニメーション
		let delta = PLAYER_MOVE_COEF * time_delta.as_secs_f32();
		let locate = &mut transform.translation;
		match player.direction
		{	Direction::Up    => locate.y += delta,
			Direction::Left  => locate.x -= delta,
			Direction::Right => locate.x += delta,
			Direction::Down  => locate.y -= delta,
		}
		player.sprite_location = ( locate.x, locate.y );

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
		let locate = &mut transform.translation;
		locate.x = sprite_x;
		locate.y = sprite_y;
		player.sprite_location = ( locate.x, locate.y );

		//スプライト(三角形)の表示向きを更新する
		if player.direction != player.new_direction
		{	let angle = decide_angle( player.direction, player.new_direction );
			let quat = Quat::from_rotation_z( angle.to_radians() );
			transform.rotate( quat );
			player.direction = player.new_direction;
		}

		//ドット獲得判定
		if let MapObj::Dot1( opt_dot ) = stage.map[ map_x ][ map_y ]
		{	cmds.entity( opt_dot.unwrap() ).despawn();
			stage.map[ map_x ][ map_y ] = MapObj::Space;
			record.score += 1;
		}

		//ゴールしたので、eventをセットして関数から脱出
		if ( map_x, map_y ) == stage.goal_xy
		{	event.send( GameState::GameClear );
			return;
		}

		//上下左右にあるものを取り出す
		let ( up, left, right, down ) = get_map_obj_ulrd( ( map_x, map_y ), &stage );

		//キー入力を取得する
		let key_left  = inkey.pressed( KeyCode::Left  );
		let key_right = inkey.pressed( KeyCode::Right );
		let key_up    = inkey.pressed( KeyCode::Up    );
		let key_down  = inkey.pressed( KeyCode::Down  );

		//カーソルキーの入力により自機の向きを変える
		if key_left
		{	player.new_direction = Direction::Left;
			player.stop = matches!( left, MapObj::Wall(_) );
			if ! player.stop { map_x -= 1 }
		}
		else if key_right
		{	player.new_direction = Direction::Right;
			player.stop = matches!( right, MapObj::Wall(_) );
			if ! player.stop { map_x += 1 }
		}
		else if key_up
		{	player.new_direction = Direction::Up;
			player.stop = matches!( up, MapObj::Wall(_) );
			if ! player.stop { map_y -= 1 }
		}
		else if key_down
		{	player.new_direction = Direction::Down;
			player.stop = matches!( down, MapObj::Wall(_) );
			if ! player.stop { map_y += 1 }
		}
		else
		{	player.stop = true
		}
		player.map_location = ( map_x, map_y );

		//ウェイトをリセットする
		player.wait.reset();
	}
}

//マップの上下左右にあるものを取り出す
pub fn get_map_obj_ulrd
(	( x, y ): ( usize, usize ),
	stage: &GameStage
) -> ( MapObj, MapObj, MapObj, MapObj )
{	let get_map_obj = | dx, dy | { stage.map[( x as i32 + dx ) as usize][( y as i32 + dy ) as usize] };

	let up    = if y >= 1     { get_map_obj(  0, -1 ) } else { MapObj::Wall( None ) };
	let left  = if x >= 1     { get_map_obj( -1,  0 ) } else { MapObj::Wall( None ) };
	let right = if x <= MAX_X { get_map_obj(  1,  0 ) } else { MapObj::Wall( None ) };
	let down  = if y <= MAX_Y { get_map_obj(  0,  1 ) } else { MapObj::Wall( None ) };

	( up, left, right, down )
}

//自機(三角形)の新旧の向きから、表示角度差分を決める
fn decide_angle( old: Direction, new: Direction ) -> f32
{	match old
	{	Direction::Up =>
		{	if matches!( new, Direction::Left  ) { return  90. }
			if matches!( new, Direction::Right ) { return -90. }
		}
		Direction::Left =>
		{	if matches!( new, Direction::Down  ) { return  90. }
			if matches!( new, Direction::Up    ) { return -90. }
		}
		Direction::Right =>
		{	if matches!( new, Direction::Up    ) { return  90. }
			if matches!( new, Direction::Down  ) { return -90. }
		}
		Direction::Down =>
		{	if matches!( new, Direction::Right ) { return  90. }
			if matches!( new, Direction::Left  ) { return -90. }
		}
	}

	//呼出側でold != newが保証されているので、±90°以外はすべて180°
	180.
}

////////////////////////////////////////////////////////////////////////////////

//自機のスプライトを削除する
pub fn despawn_sprite_player
(	mut q_player: Query<Entity, With<Player>>,
	mut cmds: Commands,
)
{	let player = q_player.single_mut().unwrap();
	cmds.entity( player ).despawn();
}

//End of code.