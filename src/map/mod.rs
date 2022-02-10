use super::*;

//internal modules
mod dig_and_dig_and_dig;			//迷路作成
mod dig_and_back_and_dig;			//迷路作成
mod find_and_destroy_digable_walls;	//迷路作成
mod map_utilities;					//構造解析

//Pluginの手続き
pub struct PluginMap;
impl Plugin for PluginMap
{	fn build( &self, app: &mut App )
	{	app
		//------------------------------------------------------------------------------------------
		.init_resource::<GameMap>()								// MAP情報のResource
		//==========================================================================================
		.add_system_set											// ＜GameState::Start＞
		(	SystemSet::on_enter( GameState::Start )				// ＜on_enter()＞
				.with_system( generate_new_map )				// 新マップ作成⇒GameState::Playへ
		)
		.add_system_set											// ＜GameState::Start＞
		(	SystemSet::on_exit( GameState::Start )				// ＜on_exit()＞
				.with_system( spawn_sprite_map )				// 新マップのスプライト表示
		)
		//==========================================================================================
		.add_system_set											// ＜GameState::Play＞
		(	SystemSet::on_update( GameState::Play )				// ＜on_update()＞
				.with_system( rotate_sprite_goal )				// ゴールスプライトのアニメーション
		)
		//==========================================================================================
		.add_system_set											// ＜GameState::Clear＞
		(	SystemSet::on_exit( GameState::Clear )				// ＜on_exit()＞
				.with_system( despawn_entity::<SpriteWall> )	// マップを削除(壁)
				.with_system( despawn_entity::<SpriteGoal> )	// マップを削除(ゴール)
				.with_system( despawn_entity::<SpriteCoin> )	// マップを削除(コイン)
				.with_system( despawn_entity::<DebugSprite> )	// マップを削除(デバッグ用)
		)
		//==========================================================================================
		.add_system_set											// ＜GameState::Over＞
		(	SystemSet::on_exit( GameState::Over )				// ＜on_exit()＞
				.with_system( despawn_entity::<SpriteWall> )	// マップを削除(壁)
				.with_system( despawn_entity::<SpriteGoal> )	// マップを削除(ゴール)
				.with_system( despawn_entity::<SpriteCoin> )	// マップを削除(コイン)
				.with_system( despawn_entity::<DebugSprite> )	// マップを削除(デバッグ用)
		)
		//------------------------------------------------------------------------------------------
		;
	}
}

////////////////////////////////////////////////////////////////////////////////////////////////////

//迷路生成関数の選択
#[allow(dead_code)]
#[derive(PartialEq)]
enum SelectMazeType { Random, Type1, Type2, Type3 }

const SELECT_MAZE_TYPE: SelectMazeType = SelectMazeType::Random;
//const SELECT_MAZE_TYPE: SelectMazeType = SelectMazeType::Type2;

//Sprite
#[derive(Component)]
struct SpriteWall;
const WALL_PIXEL: f32 = PIXEL_PER_GRID;

#[derive(Component)]
struct SpriteCoin;
const COIN_PIXEL: f32 = PIXEL_PER_GRID * 0.8;

#[derive(Component)]
struct SpriteGoal;
const GOAL_PIXEL: f32 = PIXEL_PER_GRID / 2.0;
const GOAL_COLOR: Color = Color::YELLOW;

#[derive(Component)]
struct DebugSprite;
const DEBUG_PIXEL: f32 = PIXEL_PER_GRID;

////////////////////////////////////////////////////////////////////////////////////////////////////

//新しい迷路を作りGameState::Playへ遷移
fn generate_new_map
(	mut maze: ResMut<GameMap>,
	mut state: ResMut<State<GameState>>,
	o_record: Option<ResMut<Record>>,
)
{	//初期化する
	maze.clear_map();
	if let Some ( mut record ) = o_record { record.stage += 1 };

	//入口を掘る
	let x = maze.rng().gen_range( RANGE_MAP_INNER_X );
	let grid = MapGrid { x, y: MAP_HEIGHT - 1 };
	*maze.start_mut() = grid;
	*maze.mapobj_mut( grid	    ) = MapObj::DeadEnd; //入口は行き止まり扱い
	*maze.mapobj_mut( grid + UP ) = MapObj::Passage; //入口直上は無条件で道

	//迷路作成関数を乱数で決め、迷路を掘らせる
	let maze_type = match SELECT_MAZE_TYPE
	{	SelectMazeType::Random => maze.rng().gen_range( 0..3 ),
		SelectMazeType::Type1  => 0,
		SelectMazeType::Type2  => 1,
		SelectMazeType::Type3  => 2,
	};
	match maze_type
	{	0 => maze.dig_and_dig_and_dig(),			//一型迷路
		1 => maze.dig_and_back_and_dig(),			//二型迷路
		2 => maze.find_and_destroy_digable_walls(),	//三型迷路
		_ => {}
	}

	//出口を掘れる場所を探し、乱数で決める
	let mut exit_x = Vec::new();
	let mut grid = MapGrid{ x: 0, y: 1 };
	RANGE_MAP_INNER_X.for_each( | x |
	{	grid.x = x;
		if ! maze.is_wall( grid ) { exit_x.push( x ) }
	} );
	let x = exit_x[ maze.rng().gen_range( 0..exit_x.len() ) ];
	grid = MapGrid { x, y: 0 };
	*maze.goal_mut() = grid;
	*maze.mapobj_mut( grid ) = MapObj::Goal ( None );

	//迷路の構造解析
	maze.identify_halls_and_passages();	//広間と通路を識別して袋小路に目印を付ける
	maze.put_coins_at_deadend();		//袋小路にコインを置く

	//Playへ遷移する
	let _ = state.overwrite_set( GameState::Play );
}

//迷路のスプライトをspawnして必要ならEntity IDを記録する
fn spawn_sprite_map
(	mut maze: ResMut<GameMap>,
	mut cmds: Commands,
	asset_svr: Res<AssetServer>,
)
{	for x in RANGE_MAP_X
	{	for y in RANGE_MAP_Y
		{	let grid = MapGrid { x, y };
			let pixel = grid.into_pixel();
			match maze.mapobj( grid )
			{	MapObj::Goal ( _ ) =>
				{	//	ゴールのスプライトを表示する 
					let custom_size = Some( Vec2::new( GOAL_PIXEL, GOAL_PIXEL ) );
					let position = Vec3::new( pixel.x, pixel.y, SPRITE_DEPTH_MAZE );
					let quat = Quat::from_rotation_z( 45_f32.to_radians() ); //45°傾ける
					let id = cmds.spawn_bundle( SpriteBundle::default() )
						.insert( Sprite { color: GOAL_COLOR, custom_size, ..Default::default() } )
						.insert( Transform::from_translation( position ).with_rotation( quat ) )
						.insert( SpriteGoal )
						.id(); 
					*maze.mapobj_mut( grid ) = MapObj::Goal ( Some ( id ) );
				}
				MapObj::Wall =>
				{	//壁のストライプを表示する
					let custom_size = Some( Vec2::new( WALL_PIXEL, WALL_PIXEL ) );
					cmds.spawn_bundle( SpriteBundle::default() )
						.insert( Sprite { custom_size, ..Default::default() } )
						.insert( asset_svr.load( IMAGE_SPRITE_WALL ) as Handle<Image> )
						.insert( Transform::from_translation( Vec3::new( pixel.x, pixel.y, SPRITE_DEPTH_MAZE ) ) )
						.insert( SpriteWall );
				}
				MapObj::Coin ( _, coin ) =>
				{	//コインのスプライトを表示する
					let custom_size = Some( Vec2::new( COIN_PIXEL, COIN_PIXEL ) );
					let id = cmds.spawn_bundle( SpriteBundle::default() )
						.insert( Sprite { custom_size, ..Default::default() } )
						.insert( asset_svr.load( IMAGE_SPRITE_COIN ) as Handle<Image> )
						.insert( Transform::from_translation( Vec3::new( pixel.x, pixel.y, SPRITE_DEPTH_MAZE ) ) )
						.insert( SpriteCoin )
						.id();
					*maze.mapobj_mut( grid ) = MapObj::Coin ( Some ( id ), coin );
				}
				_ => {}
			};

			//デバッグ用に広間のスプライトを表示する
			if cfg!( debug_assertions ) && maze.is_hall( grid )
			{	let custom_size = Some( Vec2::new( DEBUG_PIXEL, DEBUG_PIXEL ) * 0.9 );
				cmds.spawn_bundle( SpriteBundle::default() )
					.insert( Sprite { color: Color::INDIGO, custom_size, ..Default::default() } )
					.insert( Transform::from_translation( Vec3::new( pixel.x, pixel.y, SPRITE_DEPTH_DEBUG ) ) )
					.insert( DebugSprite );
			}
		}
	}
}

//ゴールのスプライトをアニメーションさせる
fn rotate_sprite_goal
(	mut q: Query<( &mut Transform, &mut Sprite ), With<SpriteGoal>>,
	time: Res<Time>,
)
{	let ( mut transform, mut sprite ) = q.get_single_mut().unwrap();

	//回転させる
	let angle = 360.0 * time.delta().as_secs_f32();
	let quat = Quat::from_rotation_z( angle.to_radians() );
	transform.rotate( quat );

	//色を変える
	let hue = ( ( time.seconds_since_startup() * 500. ) as usize % 360 ) as f32;
	let ( saturation, lightness, alpha ) = ( 1., 0.5, 1. );
	sprite.color = Color::Hsla{ hue, saturation, lightness, alpha };
}

//End of code.