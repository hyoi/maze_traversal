use super::*;

//external modules
use bevy_prototype_lyon::prelude::*;

//Pluginの手続き
pub struct PluginPlayer;
impl Plugin for PluginPlayer
{	fn build( &self, app: &mut App )
	{	app
		//------------------------------------------------------------------------------------------
		.add_plugin( ShapePlugin )								// bevy_prototype_lyon
		.init_resource::<Record>()								// スコア等のResource
		//==========================================================================================
		.add_system_set											// ＜GameState::Start＞
		(	SystemSet::on_exit( GameState::Start )				// ＜on_exit()＞
				.with_system( spawn_sprite_player )				// マップ生成後に自機を配置
		)
		//==========================================================================================
		.add_system_set											// ＜GameState::Play＞
		(	SystemSet::on_update( GameState::Play )				// ＜on_update()＞
				.with_system( move_sprite_player )				// 自機の移動、ゴール⇒GameState::Clearへ
		)
		//==========================================================================================
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
				.with_system( despawn_entity::<Player> )		// 自機を削除
				.with_system( hide_ui::<MessageClear> )			// CLEARメッセージを隠す
		)
		//------------------------------------------------------------------------------------------
		;
	}
}

////////////////////////////////////////////////////////////////////////////////////////////////////

//Sprite
const PLAYER_PIXEL: f32   = PIXEL_PER_GRID / 2.5;
const PLAYER_COLOR: Color = Color::YELLOW;

//移動ウェイト
const PLAYER_WAIT: f32 = 0.09;

//スプライトの動きを滑らかにするための中割係数
const PLAYER_MOVE_COEF: f32 = PIXEL_PER_GRID / PLAYER_WAIT;

//Default
impl Default for Player
{	fn default() -> Self
	{	Self
		{	map_xy   : MapGrid::default(),
			direction: FourSides::Up,
			key_input: FourSides::Up,
			wait: Timer::from_seconds( PLAYER_WAIT, false ),
			stop: true,
		}
	}
}

////////////////////////////////////////////////////////////////////////////////////////////////////

//自機のスプライトを初期位置に配置する
fn spawn_sprite_player( maze: Res<GameMap>, mut cmds: Commands )
{	let pixel = maze.start_xy.into_pixel();

	let triangle = &shapes::RegularPolygon
	{	sides: 3,
		feature: shapes::RegularPolygonFeature::Radius( PLAYER_PIXEL ),
		..shapes::RegularPolygon::default()
	};
	let drawmode = DrawMode::Fill( FillMode { options: FillOptions::default(), color: PLAYER_COLOR } );
	let transform = Transform::from_translation( Vec3::new( pixel.x, pixel.y, SPRITE_DEPTH_PLAYER ) );

	cmds.spawn_bundle( GeometryBuilder::build_as( triangle, drawmode, transform ) )
		.insert( Player { map_xy: maze.start_xy, ..Default::default() } );
}

//自機のスプライトを移動する
fn move_sprite_player
(	mut q: Query<( &mut Player, &mut Transform )>,
	o_record: Option<ResMut<Record>>,
	( mut maze, mut state ): ( ResMut<GameMap>, ResMut<State<GameState>> ),
	( mut cmds, time, inkey ): ( Commands, Res<Time>, Res<Input<KeyCode>> ),
)
{	let time_delta = time.delta();
	let ( mut player, mut transform ) = q.get_single_mut().unwrap();

	if player.wait.tick( time_delta ).finished()
	{	//スプライトの表示位置をグリッドに合わせて更新する
		let mut grid = player.map_xy;
		let pixel = grid.into_pixel();
		let position = &mut transform.translation;
		position.x = pixel.x;
		position.y = pixel.y;

		//自機のの表示向きを更新する
		if player.direction != player.key_input
		{	rotate_player_sprite( &player, &mut transform );
			player.direction = player.key_input;
		}

		//ゴールドを拾う
		if maze.is_dead_end( grid )
		{	if let MapObj::Coin ( Some( id ) ) = maze.map( grid )
			{	if let Some( mut record ) = o_record { record.score += maze.coin( grid ) }
				maze.coin[ grid.x ][ grid.y ] = 0;
				maze.map [ grid.x ][ grid.y ] = MapObj::Pathway;
				cmds.entity( id ).despawn();
			}
		}

		//ゴールしたら、Clearへ遷移する
		if grid == maze.goal_xy
		{	if let MapObj::Goal ( Some( id ) ) = maze.map( grid )
			{	cmds.entity( id ).despawn();
			}
			let _ = state.overwrite_set( GameState::Clear );
			return;
		}

		//キー入力を取得する
		let key_left  = inkey.pressed( KeyCode::Left  );
		let key_right = inkey.pressed( KeyCode::Right );
		let key_up    = inkey.pressed( KeyCode::Up    );
		let key_down  = inkey.pressed( KeyCode::Down  );

		//キー入力により自機の向きを変える(スプライトの回転はまだ)
		if key_left
		{	player.key_input = FourSides::Left;
			player.stop = maze.is_wall_middle_left( grid );
			if ! player.stop { grid.x -= 1 }
		}
		else if key_right
		{	player.key_input = FourSides::Right;
			player.stop = maze.is_wall_middle_right( grid );
			if ! player.stop { grid.x += 1 }
		}
		else if key_up
		{	player.key_input = FourSides::Up;
			player.stop = maze.is_wall_upper_center( grid );
			if ! player.stop { grid.y -= 1 }
		}
		else if key_down
		{	player.key_input = FourSides::Down;
			player.stop = maze.is_wall_lower_center( grid );
			if ! player.stop { grid.y += 1 }
		}
		else
		{	player.stop = true
		}
		player.map_xy = grid;

		//ウェイトをリセットする
		player.wait.reset();
	}
	else if ! player.stop
	{	//スプライトを滑らかに移動させるための中割アニメーション
		let delta = PLAYER_MOVE_COEF * time_delta.as_secs_f32();
		let position = &mut transform.translation;
		match player.direction
		{	FourSides::Up    => position.y += delta,
			FourSides::Left  => position.x -= delta,
			FourSides::Right => position.x += delta,
			FourSides::Down  => position.y -= delta,
		}

		//自機のの表示向きを更新する
		if player.direction != player.key_input
		{	rotate_player_sprite( &player, &mut transform );
			player.direction = player.key_input;
		}
	}
}

//現在の自機の向きとキー入力から角度の差分を求めて、自機を回転させる
fn rotate_player_sprite( player: &Player, transform: &mut Mut<Transform> )
{	let angle: f32 = match player.direction
	{	FourSides::Up =>
		{	if      player.key_input.is_left()  {  90.0 }
			else if player.key_input.is_right() { -90.0 }
			else    { 180.0 }
		}
		FourSides::Left =>
		{	if      player.key_input.is_down() {  90.0 }
			else if player.key_input.is_up()   { -90.0 }
			else    { 180.0 }
		}
		FourSides::Right =>
		{	if      player.key_input.is_up()   {  90.0 }
			else if player.key_input.is_down() { -90.0 }
			else    { 180.0 }
		}
		FourSides::Down =>
		{	if      player.key_input.is_right() {  90.0 }
			else if player.key_input.is_left()  { -90.0 }
			else    { 180.0 }
		}
	};

	let quat = Quat::from_rotation_z( angle.to_radians() );
	transform.rotate( quat );
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