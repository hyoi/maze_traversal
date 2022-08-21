use super::*;

//external modules
use bevy::sprite::MaterialMesh2dBundle;

//Pluginの手続き
pub struct PluginPlayer;
impl Plugin for PluginPlayer
{	fn build( &self, app: &mut App )
	{	app
		//------------------------------------------------------------------------------------------
		.init_resource::<Record>()									// スコア等のResource
		//==========================================================================================
		.add_system_set												// ＜GameState::Start＞
		(	SystemSet::on_exit( GameState::Start )					// ＜on_exit()＞
				.with_system( spawn_sprite_player )					// マップ生成後に自機を配置
		)
		//==========================================================================================
		.add_system_set												// ＜GameState::Play＞
		(	SystemSet::on_update( GameState::Play )					// ＜on_update()＞
				.with_system( move_sprite_player )					// 自機の移動、ゴール⇒GameState::Clearへ
		)
		//==========================================================================================
		.add_system_set												// ＜GameState::Clear＞
		(	SystemSet::on_enter( GameState::Clear )					// ＜on_enter()＞
				.with_system( show_ui::<MessageClear> )				// CLEARメッセージを表示する
		)
		.add_system_set												// ＜GameState::Clear＞
		(	SystemSet::on_update( GameState::Clear )				// ＜on_update()＞
				.with_system( countdown_to_start::<MessageClear> )	// CD完了⇒GameState::Startへ
		)
		.add_system_set												// ＜GameState::Clear＞
		(	SystemSet::on_exit( GameState::Clear )					// ＜on_exit()＞
				.with_system( despawn_entity::<Player> )			// 自機を削除
				.with_system( hide_ui::<MessageClear> )				// CLEARメッセージを隠す
		)
		//==========================================================================================
		.add_system_set												// ＜GameState::Over＞
		(	SystemSet::on_exit( GameState::Over )					// ＜on_exit()＞
				.with_system( despawn_entity::<Player> )			// 自機を削除
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
		{	grid: MapGrid::default(),
			side: UP,
			key_input: UP,
			wait: Timer::from_seconds( PLAYER_WAIT, false ),
			stop: true,
		}
	}
}

////////////////////////////////////////////////////////////////////////////////////////////////////

//自機のスプライトを初期位置に配置する
fn spawn_sprite_player
(	maze: Res<GameMap>,
	mut cmds: Commands,
	mut meshes: ResMut<Assets<Mesh>>,
	mut materials: ResMut<Assets<ColorMaterial>>,
)
{	let pixel     = maze.start().into_pixel();
	let transform = Transform::from_translation( Vec3::new( pixel.x, pixel.y, SPRITE_DEPTH_PLAYER ) );
	let mesh	  = meshes.add( shape::RegularPolygon::new( PLAYER_PIXEL, 3 ).into() ).into();
	let material  = materials.add( ColorMaterial::from( PLAYER_COLOR ) );

	let sprite = MaterialMesh2dBundle { transform, mesh, material, ..default() };
	cmds.spawn_bundle( sprite ).insert( Player { grid: maze.start(), ..default() } );
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
		let mut grid = player.grid;
		let pixel = grid.into_pixel();
		let position = &mut transform.translation;
		position.x = pixel.x;
		position.y = pixel.y;

		//自機のの表示向きを更新する
		if player.side != player.key_input
		{	rotate_player_sprite( &player, &mut transform );
			player.side = player.key_input;
		}

		//ゴールドを拾う
		if let MapObj::Coin ( Some( id ), coin ) = maze.mapobj( grid )
		{	if let Some( mut record ) = o_record { record.score += coin }
			*maze.mapobj_mut( grid ) = MapObj::Passage;
			cmds.entity( id ).despawn();
		}

		//ゴールしたら、Clearへ遷移する
		if grid == maze.goal()
		{	if let MapObj::Goal ( Some( id ) ) = maze.mapobj( grid )
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
		{	player.key_input = LEFT;
			player.stop = maze.is_wall( grid + LEFT );
			if ! player.stop { grid.x -= 1 }
		}
		else if key_right
		{	player.key_input = RIGHT;
			player.stop = maze.is_wall( grid + RIGHT );
			if ! player.stop { grid.x += 1 }
		}
		else if key_up
		{	player.key_input = UP;
			player.stop = maze.is_wall( grid + UP );
			if ! player.stop { grid.y -= 1 }
		}
		else if key_down
		{	player.key_input = DOWN;
			player.stop = maze.is_wall( grid + DOWN );
			if ! player.stop { grid.y += 1 }
		}
		else
		{	player.stop = true
		}
		player.grid = grid;

		//ウェイトをリセットする
		player.wait.reset();
	}
	else if ! player.stop
	{	//スプライトを滑らかに移動させるための中割アニメーション
		let delta = PLAYER_MOVE_COEF * time_delta.as_secs_f32();
		let position = &mut transform.translation;
		match player.side
		{	UP    => position.y += delta,
			LEFT  => position.x -= delta,
			RIGHT => position.x += delta,
			DOWN  => position.y -= delta,
		}

		//自機のの表示向きを更新する
		if player.side != player.key_input
		{	rotate_player_sprite( &player, &mut transform );
			player.side = player.key_input;
		}
	}
}

//現在の自機の向きとキー入力から角度の差分を求めて、自機を回転させる
fn rotate_player_sprite( player: &Player, transform: &mut Mut<Transform> )
{	let angle: f32 = match player.side
	{	UP =>
		{	if      player.key_input == LEFT  {  90.0 }
			else if player.key_input == RIGHT { -90.0 }
			else    { 180.0 }
		}
		LEFT =>
		{	if      player.key_input == DOWN {  90.0 }
			else if player.key_input == UP   { -90.0 }
			else    { 180.0 }
		}
		RIGHT =>
		{	if      player.key_input == UP   {  90.0 }
			else if player.key_input == DOWN { -90.0 }
			else    { 180.0 }
		}
		DOWN =>
		{	if      player.key_input == RIGHT {  90.0 }
			else if player.key_input == LEFT  { -90.0 }
			else    { 180.0 }
		}
	};

	let quat = Quat::from_rotation_z( angle.to_radians() );
	transform.rotate( quat );
}

//End of code.