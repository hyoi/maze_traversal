use super::*;

//external modules
use bevy_prototype_lyon::{ prelude::*, entity::ShapeBundle };

//Pluginの手続き
pub struct PluginPlayer;
impl Plugin for PluginPlayer
{	fn build( &self, app: &mut App )
	{	app
		//------------------------------------------------------------------------------------------
		.add_plugin( ShapePlugin )								// bevy_prototype_lyon
		.init_resource::<Record>()					// 全体に影響する変数を格納するResource
		//==========================================================================================
		.add_system_set											// ＜GameState::Start＞
		(	SystemSet::on_exit( GameState::Start )				// ＜on_exit()＞
				.with_system( spawn_sprite_player )				// マップ生成後に自機を配置
		)
		//------------------------------------------------------------------------------------------
		.add_system_set											// ＜GameState::Play＞
		(	SystemSet::on_update( GameState::Play )				// ＜on_update()＞
				.with_system( move_sprite_player )				// 自機の移動、ゴール⇒GameState::Clearへ
		)
		//------------------------------------------------------------------------------------------
		.add_system_set											// ＜GameState::Clear＞
		(	SystemSet::on_enter( GameState::Clear )				// ＜on_enter()＞
				.with_system( show_ui::<MessageClear> )			// CLEARメッセージを表示する
		)
		.add_system_set											// ＜GameState::Clear＞
		(	SystemSet::on_update( GameState::Clear )			// ＜on_update()＞
				.with_system( change_state_after_countdown )	// CD完了⇒GameState::Startへ
		)
		.add_system_set											// ＜GameState::Clear＞
		(	SystemSet::on_exit( GameState::Clear )				// ＜on_exit()＞
			.with_system( despawn_entity::<Player> )			// 自機を削除
			.with_system( hide_ui::<MessageClear> )				// CLEARメッセージを隠す
		)
		//------------------------------------------------------------------------------------------
		;
	}
}

////////////////////////////////////////////////////////////////////////////////////////////////////

//自機のスプライトを初期位置に配置する
fn spawn_sprite_player( maze: Res<GameMap>, mut cmds: Commands )
{	let ( map_x, map_y ) = maze.start_xy;
	let ( sprite_x, sprite_y ) = into_pixel_xy( map_x, map_y );

	let player = Player
	{	map_postion: ( map_x, map_y ),
		sprite_postion: ( sprite_x, sprite_y ),
		..Default::default()
	};

	//スプライトを初期位置に配置する
	let sprite = sprite_player( player.sprite_postion );
	cmds.spawn_bundle( sprite ).insert( player );
}

//自機のスプライトバンドルを生成
fn sprite_player( ( x, y ): ( f32, f32 ) ) -> ShapeBundle
{	let triangle = &shapes::RegularPolygon
	{	sides: 3,
		feature: shapes::RegularPolygonFeature::Radius( PLAYER_PIXEL ),
		..shapes::RegularPolygon::default()
	};
	let drawmode  = DrawMode::Fill( FillMode { options: FillOptions::default(), color: PLAYER_COLOR } );
	let transform = Transform::from_translation( Vec3::new( x, y, SPRITE_DEPTH_PLAYER ) );

	GeometryBuilder::build_as( triangle, drawmode, transform )
}

////////////////////////////////////////////////////////////////////////////////////////////////////

//自機のスプライトを移動する
fn move_sprite_player
(	mut q: Query<( &mut Player, &mut Transform )>,
	mut state : ResMut<State<GameState>>,
	o_record: Option<ResMut<Record>>,
	mut maze: ResMut<GameMap>,
	( mut cmds, time, inkey ): ( Commands, Res<Time>, Res<Input<KeyCode>> ),
)
{	let time_delta = time.delta();
	let ( mut player, mut transform ) = q.get_single_mut().unwrap();

	use common::types::Direction::*;

	if ! player.wait.tick( time_delta ).finished()
	{	//停止中なら何も処理しない
		if player.stop { return }

		//スプライトを滑らかに移動させるための中割アニメーション
		let delta = PLAYER_MOVE_COEF * time_delta.as_secs_f32();
		let position = &mut transform.translation;
		match player.direction
		{	Up    => position.y += delta,
			Left  => position.x -= delta,
			Right => position.x += delta,
			Down  => position.y -= delta,
		}
		player.sprite_postion = ( position.x, position.y );

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
		let ( mut map_x, mut map_y ) = player.map_postion;
		let ( sprite_x, sprite_y ) = into_pixel_xy( map_x, map_y );
		let position = &mut transform.translation;
		position.x = sprite_x;
		position.y = sprite_y;
		player.sprite_postion = ( position.x, position.y );

		//スプライト(三角形)の表示向きを更新する
		if player.direction != player.new_direction
		{	let angle = decide_angle( player.direction, player.new_direction );
			let quat = Quat::from_rotation_z( angle.to_radians() );
			transform.rotate( quat );
			player.direction = player.new_direction;
		}

		//ゴールしたら、Clearへ遷移する
		if ( map_x, map_y ) == maze.goal_xy
		{	if let MapObj::Goal ( Some( id ) ) = maze.map[ map_x as usize ][ map_y as usize ]
			{	cmds.entity( id ).despawn();
			}
			let _ = state.overwrite_set( GameState::Clear );
			return;
		}

		//ゴールドを拾う
		if maze.is_dead_end( map_x, map_y )
		{	if let Some ( mut record ) = o_record
			{	record.score += maze.count[ map_x ][ map_y ];
				maze.count[ map_x ][ map_y ] = 0;
				if let MapObj::Coin ( Some( id ) ) = maze.map[ map_x ][ map_y ]
				{	cmds.entity( id ).despawn_recursive();
					maze.map[ map_x ][ map_y ] = MapObj::Pathway;
				}
			}
		}

		//キー入力を取得する
		let key_left  = inkey.pressed( KeyCode::Left  );
		let key_right = inkey.pressed( KeyCode::Right );
		let key_up    = inkey.pressed( KeyCode::Up    );
		let key_down  = inkey.pressed( KeyCode::Down  );

		//カーソルキーの入力により自機の向きを変える
		if key_left
		{	player.new_direction = Left;
			player.stop = maze.is_wall_middle_left( map_x, map_y );
			if ! player.stop { map_x -= 1 }
		}
		else if key_right
		{	player.new_direction = Right;
			player.stop = maze.is_wall_middle_right( map_x, map_y );
			if ! player.stop { map_x += 1 }
		}
		else if key_up
		{	player.new_direction = Up;
			player.stop = maze.is_wall_upper_center( map_x, map_y );
			if ! player.stop { map_y -= 1 }
		}
		else if key_down
		{	player.new_direction = Down;
			player.stop = maze.is_wall_lower_center( map_x, map_y );
			if ! player.stop { map_y += 1 }
		}
		else
		{	player.stop = true
		}
		player.map_postion = ( map_x, map_y );

		//ウェイトをリセットする
		player.wait.reset();
	}
}

//自機(三角形)の新旧の向きから、表示角度差分を決める
fn decide_angle( old: common::types::Direction, new: common::types::Direction ) -> f32
{	use common::types::Direction::*;

	match old
	{	Up =>
		{	if matches!( new, Left  ) { return  90.0 }
			if matches!( new, Right ) { return -90.0 }
		}
		Left =>
		{	if matches!( new, Down  ) { return  90.0 }
			if matches!( new, Up    ) { return -90.0 }
		}
		Right =>
		{	if matches!( new, Up    ) { return  90.0 }
			if matches!( new, Down  ) { return -90.0 }
		}
		Down =>
		{	if matches!( new, Right ) { return  90.0 }
			if matches!( new, Left  ) { return -90.0 }
		}
	}

	//呼出側でold != newが保証されているので、±90°以外はすべて180°
	180.0
}

////////////////////////////////////////////////////////////////////////////////////////////////////

//カウントダウンの後、Startへ遷移
fn change_state_after_countdown
(	mut q: Query<&mut Text, With<MessageClear>>,
	mut state: ResMut<State<GameState>>,
	time: Res<Time>,
	( mut count, mut timer ): ( Local<i32>, Local<Timer> ),
)
{	if let Ok( mut ui ) = q.get_single_mut()
	{	if *count <= 0									//カウンターが未初期化か？
		{	*timer = Timer::from_seconds( 1.0, false );	//1秒タイマーセット
			*count = 6;									//カウント数の初期化
		}
		else if timer.tick( time.delta() ).finished()	//1秒経過したら
		{	timer.reset();								//タイマー再セット
			*count -= 1;								//カウントダウン

			//カウントダウンが終わったら、Startへ遷移する
			if *count <= 0 { let _ = state.overwrite_set( GameState::Start ); }
		}
		ui.sections[ 2 ].value = ( *count - 1 ).max( 0 ).to_string();
	}
}

//End of code.