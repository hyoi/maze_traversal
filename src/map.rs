use super::*;

//internal modules
mod util;
pub use util::*;

mod dig_and_dig_and_dig;
use dig_and_dig_and_dig::*;				//迷路作成関数
mod dig_and_back_and_dig;
use dig_and_back_and_dig::*;			//迷路作成関数
mod find_and_destroy_digable_walls;
use find_and_destroy_digable_walls::*;	//迷路作成関数

mod find_passageway;
use find_passageway::*;

////////////////////////////////////////////////////////////////////////////////////////////////////

//Pluginの手続き
pub struct PluginMap;
impl Plugin for PluginMap
{	fn build( &self, app: &mut AppBuilder )
	{	app
		//------------------------------------------------------------------------------------------
		.init_resource::<GameMap>()							// MAP情報のResource
		//------------------------------------------------------------------------------------------
		.add_system_set											// GameState::Start
		(	SystemSet::on_enter( GameState::Start )				// on_enter()
				.with_system( spawn_sprite_new_map.system() )	// 新マップ表示⇒GameState::Playへ
		)
		//------------------------------------------------------------------------------------------
		.add_system_set											// GameState::Play
		(	SystemSet::on_update( GameState::Play )				// on_update()
				.with_system( animate_goal_sprite.system() )	// ゴールスプライトのアニメーション
		)
		//------------------------------------------------------------------------------------------
		.add_system_set											// GameState::Clear
		(	SystemSet::on_enter( GameState::Clear )				// on_enter()
				.with_system( show_whole_map.system() )			// 地図の全体像を見せる
		)
		.add_system_set											// GameState::Clear
		(	SystemSet::on_exit( GameState::Clear )				// on_exit()
				.with_system( despawn_sprite_map.system() )		// マップを削除
		)
		//------------------------------------------------------------------------------------------
		;
	}
}

////////////////////////////////////////////////////////////////////////////////////////////////////

//定義と定数

//迷路の縦横のマス数
pub const MAP_WIDTH : i32 = 66;
pub const MAP_HEIGHT: i32 = 36;

//MAPのマスの種類
#[derive(Copy,Clone,PartialEq)]
pub enum MapObj
{	None,
	Wall ( Option<Entity> ),
	Dot1 ( Option<Entity> ),	//通常の道
	Dot2 ( Option<Entity> ),	//行き止まり目印用
	Goal ( Option<Entity> ),
	Space,
}

//MAPのマスの状態の制御に使うbit
const BIT_ALL_CLEAR  : usize = 0b00;

const BIT1_IS_VISIBLE: usize = 0b01;
const BIT1_SHOW      : usize = 0b01;
const BIT1_HIDE      : usize = 0b00;

const BIT2_PASSAGEWAY: usize = 0b10;

//MAP情報のResource
pub struct GameMap
{	pub rng: rand::prelude::StdRng,	//再現性がある乱数を使いたいので
	pub level: usize,
	pub map : [ [ MapObj; MAP_HEIGHT as usize ]; MAP_WIDTH as usize ],
	pub stat: [ [ usize ; MAP_HEIGHT as usize ]; MAP_WIDTH as usize ],
	pub start_xy: ( i32, i32 ),
	pub goal_xy : ( i32, i32 ),
	pub count_dots: usize,
	pub is_darkmode: bool,
}
impl Default for GameMap
{	fn default() -> Self
	{	Self
		{//	rng: StdRng::seed_from_u64( rand::thread_rng().gen::<u64>() ),	//本番用
			rng: StdRng::seed_from_u64( 1234567890 ),	//開発用：再現性がある乱数を使いたい場合
			level: 0,
			map : [ [ MapObj::None ; MAP_HEIGHT as usize ]; MAP_WIDTH as usize ],
			stat: [ [ BIT_ALL_CLEAR; MAP_HEIGHT as usize ]; MAP_WIDTH as usize ],
			start_xy: ( 0, 0 ),
			goal_xy : ( 0, 0 ),
			count_dots: 0,
			is_darkmode: true,
		}
	}
}

//マップ座標の上下左右を表す定数
const UP       :   ( i32, i32 )      = (  0, -1 );
const LEFT     :   ( i32, i32 )      = ( -1,  0 );
const RIGHT    :   ( i32, i32 )      = (  1,  0 );
const DOWN     :   ( i32, i32 )      = (  0,  1 );
const DIRECTION: [ ( i32, i32 ); 4 ] = [ UP, LEFT, RIGHT, DOWN ];

//Sprite
const SPRITE_DEPTH_MAZE   : f32 = 10.0;
const SPRITE_DEPTH_SYSTILE: f32 =  5.0;

const WALL_PIXEL: f32 = PIXEL_PER_GRID;
pub const WALL_SPRITE_FILE: &str = "sprites/wall.png";

const DOT_RAIDUS: f32 = PIXEL_PER_GRID / 14.0;
const DOT_COLOR: Color = Color::WHITE;

struct GoalSprite;
const GOAL_PIXEL: f32 = PIXEL_PER_GRID / 2.0;
const GOAL_COLOR: Color = Color::YELLOW;

struct SysTileSprite;
const SYSTILE_PIXEL: f32 = PIXEL_PER_GRID;
const SYSTILE_COLOR: Color = Color::DARK_GRAY;

////////////////////////////////////////////////////////////////////////////////////////////////////

//新しい迷路を作り表示して、Playへ遷移する
fn spawn_sprite_new_map
(	mut maze: ResMut<GameMap>,
	mut state : ResMut<State<GameState>>,
	mut cmds: Commands,
	mut color_matl: ResMut<Assets<ColorMaterial>>,
	asset_svr: Res<AssetServer>,
)
{	//map配列を初期化する
	maze.map .iter_mut().for_each( | x | x.fill( MapObj::Wall( None ) ) );
	maze.stat.iter_mut().for_each( | x | x.fill( BIT_ALL_CLEAR        ) );
	maze.count_dots = 0;
	maze.level += 1;

	//入口を掘る
	let x = maze.rng.gen_range( MAP_DIGABLE_X );
	maze.map[ x as usize ][ ( MAP_HEIGHT - 2 ) as usize ] = MapObj::Dot1( None );
	maze.map[ x as usize ][ ( MAP_HEIGHT - 1 ) as usize ] = MapObj::Dot2( None ); //入口は行き止まり扱い
	maze.start_xy = ( x, MAP_HEIGHT - 1 );

	//呼び出す関数を乱数で決め、迷路を掘らせる
	match maze.rng.gen_range( 0..3 )
	{	0 => dig_and_dig_and_dig( &mut maze ),				//一型迷路
		1 => dig_and_back_and_dig( &mut maze ),				//二型迷路
		2 => find_and_destroy_digable_walls( &mut maze ),	//三型迷路
		_ => {}
	}

	//出口を掘れる場所を探し、乱数で決める
	let mut exit_x = Vec::new();
	for ( x, ary ) in maze.map.iter().enumerate() //enumerate()が生成するxの型はusize
	{	if MAP_DIGABLE_X.contains( &( x as i32 ) )
		&& ! matches!( ary[ 1 ], MapObj::Wall(_) ) { exit_x.push( x ) }
	}
	let x = exit_x[ maze.rng.gen_range( 0..exit_x.len() ) ];
	maze.map[ x ][ 0 ] = MapObj::Goal( None );
	maze.goal_xy = ( x as i32, 0 );

	//迷路の構造解析
	find_passageway( &mut maze );
	for x in MAP_DIGABLE_X
	{	for y in MAP_DIGABLE_Y
		{	if maze.stat[ x as usize ][ y as usize ] & BIT2_PASSAGEWAY != 0
			{	let xy = conv_sprite_coordinates( x, y );
				cmds.spawn_bundle( sprite_system_tile( xy, &mut color_matl ) )
					.insert( SysTileSprite );
			}
		}

	}

	//スプライトをspawnしてEntity IDを記録する
	let mut count = 0;
	let darkmode = maze.is_darkmode;
	for ( x, ary ) in maze.map.iter_mut().enumerate()
	{	for ( y, obj ) in ary.iter_mut().enumerate()
		{	let xy = conv_sprite_coordinates( x as i32, y as i32);
			match obj
			{	MapObj::Dot1(_) =>
				{	let id = cmds
						.spawn_bundle( sprite_dot( xy, &mut color_matl, darkmode ) )
						.id(); 
					*obj = MapObj::Dot1( Some( id ) );
					count += 1;
				}
				MapObj::Dot2(_) =>
				{	let id = cmds
						.spawn_bundle( sprite_dot( xy, &mut color_matl, darkmode ) )
						.id(); 
					*obj = MapObj::Dot1( Some( id ) ); //Dot2もDot1へ変換する
					count += 1;
				}
				MapObj::Goal(_) =>
				{	let id = cmds
						.spawn_bundle( sprite_goal( xy, &mut color_matl ) )
						.insert( GoalSprite )
						.id(); 
					*obj = MapObj::Goal( Some( id ) );
					count += 1;
				}
				MapObj::Wall(_) =>
				{	let id = cmds
						.spawn_bundle( sprite_wall( xy, &mut color_matl, &asset_svr, darkmode ) )
						.id();
					*obj = MapObj::Wall( Some( id ) );
				}
				_ => { *obj = MapObj::Space }
			}
		}
	}
	maze.count_dots = count;

	//Playへ遷移する
	let _ = state.overwrite_set( GameState::Play );
}

//ゴールのスプライトをアニメーションさせる
fn animate_goal_sprite
(	mut q: Query<( &mut Transform, &Handle<ColorMaterial> ), With<GoalSprite>>,
	mut color_matl: ResMut<Assets<ColorMaterial>>,
	time: Res<Time>,
)
{	let ( mut transform, handle ) = q.single_mut().unwrap();
	let color_matl = color_matl.get_mut( handle ).unwrap();

	//回転させる
	let angle = 360.0 * time.delta().as_secs_f32();
	let quat = Quat::from_rotation_z( angle.to_radians() );
	transform.rotate( quat );

	//色を変える
	let hue = ( ( time.seconds_since_startup() * 500. ) as usize % 360 ) as f32;
	let ( saturation, lightness, alpha ) = ( 1., 0.5, 1. );
	color_matl.color = Color::Hsla{ hue, saturation, lightness, alpha };
}

//地図の全体像を見せる
pub fn show_whole_map
(	mut q: Query<&mut Visible>,
	maze: ResMut<GameMap>,
)
{	for ary in maze.map.iter()
	{	for obj in ary.iter()
		{	match obj
			{	MapObj::Wall( Some( id ) ) => q.get_component_mut::<Visible>( *id ).unwrap().is_visible = true,
				MapObj::Dot1( Some( id ) ) => q.get_component_mut::<Visible>( *id ).unwrap().is_visible = true,
				_ => {}
			}
		}
	}
}

//地図のまだオープンになっていないマスを隠す
pub fn hide_whole_map
(	mut q: Query<&mut Visible>,
	maze: ResMut<GameMap>,
)
{	for ( x, ary ) in maze.map.iter().enumerate()
	{	for ( y, obj ) in ary.iter().enumerate()
		{	if maze.stat[ x ][ y ] & BIT1_IS_VISIBLE != BIT1_HIDE { continue }
			match obj
			{	MapObj::Wall( Some( id ) ) => q.get_component_mut::<Visible>( *id ).unwrap().is_visible = false,
				MapObj::Dot1( Some( id ) ) => q.get_component_mut::<Visible>( *id ).unwrap().is_visible = false,
				_ => {}
			}
		}
	}
}

//スプライトを削除する
fn despawn_sprite_map
(	maze: Res<GameMap>,
	mut cmds: Commands,
	q: Query<Entity, With<SysTileSprite>>,
)
{	for ary in maze.map.iter()
	{	for obj in ary.iter()
		{	match obj
			{	MapObj::Dot1( Some( id ) ) => cmds.entity( *id ).despawn(),
				MapObj::Wall( Some( id ) ) => cmds.entity( *id ).despawn(),
				_ => {}
			}
		}
	}
	q.for_each( | id | cmds.entity( id ).despawn() );
}

////////////////////////////////////////////////////////////////////////////////////////////////////

//ドット用のスプライトバンドルを生成
fn sprite_dot
(	( x, y ): ( f32, f32 ),
	color_matl: &mut ResMut<Assets<ColorMaterial>>,
	darkmode: bool,
) -> SpriteBundle
//) -> ShapeBundle
// {	GeometryBuilder::build_as
// 	(	&shapes::Circle { radius: DOT_RAIDUS, ..shapes::Circle::default() },
// 		ShapeColors::new( DOT_COLOR ),
//         DrawMode::Fill( FillOptions::default() ),
//         Transform::from_translation( Vec3::new( x, y, SPRITE_DEPTH_MAZE ) ),
//     )
// }
{	SpriteBundle
	{	material : color_matl.add( DOT_COLOR.into() ),
		transform: Transform::from_translation( Vec3::new( x, y, SPRITE_DEPTH_MAZE ) ),
		sprite   : Sprite::new( Vec2::new( DOT_RAIDUS, DOT_RAIDUS ) * 2.0 ),
		visible  : Visible { is_visible: ! darkmode, ..Default::default() },
		..Default::default()
	}
}

//ゴールのスプライトバンドルを生成
fn sprite_goal( ( x, y ): ( f32, f32 ), color_matl: &mut ResMut<Assets<ColorMaterial>> ) -> SpriteBundle
{	let mut sprite = SpriteBundle
	{	material : color_matl.add( GOAL_COLOR.into() ),
		transform: Transform::from_translation( Vec3::new( x, y, SPRITE_DEPTH_MAZE ) ),
		sprite   : Sprite::new( Vec2::new( GOAL_PIXEL, GOAL_PIXEL ) ),
		..Default::default()
	};
	sprite.transform.rotate( Quat::from_rotation_z( 45_f32.to_radians() ) );

	sprite
}

//壁用のスプライトバンドルを生成
fn sprite_wall
(	( x, y ): ( f32, f32 ),
	color_matl: &mut ResMut<Assets<ColorMaterial>>,
	asset_svr: &Res<AssetServer>,
	darkmode: bool,
) -> SpriteBundle
{	SpriteBundle
	{	material : color_matl.add( asset_svr.load( WALL_SPRITE_FILE ).into() ),
		transform: Transform::from_translation( Vec3::new( x, y, SPRITE_DEPTH_MAZE ) ),
		sprite   : Sprite::new( Vec2::new( WALL_PIXEL, WALL_PIXEL ) ),
		visible  : Visible { is_visible: ! darkmode, ..Default::default() },
		..Default::default()
	}
}

fn sprite_system_tile
(	( x, y ): ( f32, f32 ),
	color_matl: &mut ResMut<Assets<ColorMaterial>>,
//	darkmode: bool,
) -> SpriteBundle
{	SpriteBundle
	{	material : color_matl.add( SYSTILE_COLOR.into() ),
		transform: Transform::from_translation( Vec3::new( x, y, SPRITE_DEPTH_SYSTILE ) ),
		sprite   : Sprite::new( Vec2::new( SYSTILE_PIXEL, SYSTILE_PIXEL ) ),
//		visible  : Visible { is_visible: ! darkmode, ..Default::default() },
		..Default::default()
	}
}

//End of code.