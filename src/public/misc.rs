use super::*;

// //bevyのカメラの設置
// pub fn spawn_camera
// (	mut cmds: Commands,
//     mut meshes: ResMut<Assets<Mesh>>,
//     mut materials: ResMut<Assets<StandardMaterial>>,
// )
// {	cmds
// 	.spawn( Camera2dBundle::default() )
//     .insert( Camera { priority: 0, ..default() } )
// 	;

// 	let light = PointLightBundle
//     {   point_light: PointLight
//         {   intensity: 1500.0,
//             shadows_enabled: true,
//             ..default()
//         },
//         transform: Transform::from_xyz( 4.0, 8.0, 4.0 ),
//         ..default()
//     };

//     let plane = PbrBundle
//     {   mesh: meshes.add( Mesh::from( shape::Plane { size: MAP_GRIDS_SHARP_PLANE as f32 } ) ),
//         material: materials.add( Color::DARK_GREEN.into() ),
//         ..default()
//     }; 

//     cmds.spawn( light );
//     cmds.spawn( plane );
// }

// ComponentでQueryしたEnityを再帰的に削除する
pub fn despawn_entity<T: Component>( q: Query<Entity, With<T>>, mut cmds: Commands )
{	q.for_each( | id | cmds.entity( id ).despawn_recursive() );
}

// UI Textを表示する
pub fn show_ui<T: Component>( mut q: Query<&mut Visibility, With<T>> )
{	let _ = q.get_single_mut().map( | mut ui | ui.is_visible = true );
}

// UI Textを隠す
pub fn hide_ui<T: Component>( mut q: Query<&mut Visibility, With<T>> )
{	let _ = q.get_single_mut().map( | mut ui | ui.is_visible = false );
}

//カウントダウンの後、Startへ遷移
// pub fn countdown_to_start<T: Component>
// (	mut q: Query<&mut Text, With<T>>,
// 	mut state: ResMut<State<GameState>>,
// 	time: Res<Time>,
// 	( mut count, mut timer ): ( Local<i32>, Local<Timer> ),
// )
// {	if let Ok( mut ui ) = q.get_single_mut()
// 	{	if *count <= 0									//カウンターが未初期化か？
// 		{	*timer = Timer::from_seconds( 1.0, TimerMode::Once );	//1秒タイマーセット
// 			*count = 6;									//カウント数の初期化
// 		}
// 		else if timer.tick( time.delta() ).finished()	//1秒経過したら
// 		{	timer.reset();								//タイマー再セット
// 			*count -= 1;								//カウントダウン

// 			//カウントダウンが終わったら、Startへ遷移する
// 			if *count <= 0 { let _ = state.overwrite_set( GameState::Start ); }
// 		}
// 		ui.sections[ 2 ].value = ( *count - 1 ).max( 0 ).to_string();
// 	}
// }

//[Alt]+[Enter]でウィンドウとフルスクリーンを切り替える
#[cfg(not(target_arch = "wasm32"))]
pub fn toggle_window_mode( inkey: Res<Input<KeyCode>>, mut window: ResMut<Windows> )
{	use KeyCode::*;
	let is_alt = inkey.pressed( LAlt ) || inkey.pressed( RAlt );
	let is_alt_return = is_alt && inkey.just_pressed( Return );

	if is_alt_return
	{	use bevy::window::WindowMode::*;
		if let Some( window ) = window.get_primary_mut()
		{	let mode = if window.mode() == Windowed { SizedFullscreen } else { Windowed };
			window.set_mode( mode );
		}
	}
}

//ESCキーが入力さたら一時停止する
pub fn handle_esc_key_for_pause<T: Component>
(	mut q: Query<&mut Visibility, With<T>>,
	mut inkey: ResMut<Input<KeyCode>>,
	mut state: ResMut<State<GameState>>,
)
{	if q.get_single_mut().is_err() { return }
	if ! inkey.just_pressed( KeyCode::Escape ) { return }

	match state.current()
	{	GameState::Pause => { hide_ui( q ); state.pop().unwrap() },
		_                => { show_ui( q ); state.push( GameState::Pause ).unwrap() },
	};

	//https://bevy-cheatbook.github.io/programming/states.html#with-input
	inkey.reset( KeyCode::Escape );
}

//End of code.