use super::*;

//external modules
use rand::prelude::*;

//internal modules
mod util;
pub use util::*;

mod dig_and_dig_and_dig;			//迷路作成関数
mod dig_and_back_and_dig;			//迷路作成関数
mod find_and_destroy_digable_walls;	//迷路作成関数

mod analyze_structure;

//Pluginの手続き
pub struct PluginMap;
impl Plugin for PluginMap
{	fn build( &self, app: &mut App )
	{	app
		//------------------------------------------------------------------------------------------
		.init_resource::<GameMap>()								// MAP情報のResource
		//------------------------------------------------------------------------------------------
		.add_system_set											// ＜GameState::Start＞
		(	SystemSet::on_enter( GameState::Start )				// ＜on_enter()＞
				.with_system( spawn_sprite_new_map )			// 新マップ表示⇒GameState::Playへ
		)
		//------------------------------------------------------------------------------------------
		.add_system_set											// ＜GameState::Play＞
		(	SystemSet::on_update( GameState::Play )				// ＜on_update()＞
				.with_system( update_goal_sprite )				// ゴールスプライトのアニメーション
		)
		//------------------------------------------------------------------------------------------
		.add_system_set											// ＜GameState::Clear＞
		(	SystemSet::on_enter( GameState::Clear )				// ＜on_enter()＞
				.with_system( show_cleared_map )				// 地図の全体像を見せる
		)
		.add_system_set											// ＜GameState::Clear＞
		(	SystemSet::on_exit( GameState::Clear )				// ＜on_exit()＞
				.with_system( despawn_entity::<SpriteWall> )	// マップを削除
				.with_system( despawn_entity::<SysinfoObj> )	// マップを削除
		)
		//------------------------------------------------------------------------------------------
		;
	}
}

////////////////////////////////////////////////////////////////////////////////////////////////////

//MAPのマスの種類
#[derive(Copy,Clone,PartialEq)]
pub enum MapObj
{	None,
	Wall    ( Option<Entity> ),
	Pathway ( Option<Entity> ), //通常の道
	DeadEnd ( Option<Entity> ), //行き止まり目印用
	Goal    ( Option<Entity> ),
	Space,
}

//MAP情報のResource
pub struct GameMap
{	pub rng: rand::prelude::StdRng,	//再現性がある乱数を使いたいので
	pub map  : [ [ MapObj; MAP_HEIGHT ]; MAP_WIDTH ],
	pub stat : [ [ usize ; MAP_HEIGHT ]; MAP_WIDTH ],
	pub count: [ [ usize ; MAP_HEIGHT ]; MAP_WIDTH ],
	pub start_xy: ( usize, usize ),
	pub goal_xy : ( usize, usize ),
}
impl Default for GameMap
{	fn default() -> Self
	{	Self
		{	rng: StdRng::seed_from_u64( rand::thread_rng().gen::<u64>() ),	//本番用
		//	rng: StdRng::seed_from_u64( 1234567890 ),	//開発用：再現性がある乱数を使いたい場合
			map  : [ [ MapObj::None ; MAP_HEIGHT ]; MAP_WIDTH ],
			stat : [ [ BIT_ALL_CLEAR; MAP_HEIGHT ]; MAP_WIDTH ],
			count: [ [ 0            ; MAP_HEIGHT ]; MAP_WIDTH ],
			start_xy: ( 0, 0 ),
			goal_xy : ( 0, 0 ),
		}
	}
}

//Sprite
const SPRITE_DEPTH_MAZE   : f32 = 10.0;

#[derive(Component)]
pub struct SpriteWall { pub x: usize, pub y: usize }
const WALL_PIXEL: f32 = PIXEL_PER_GRID;

const COIN_PIXEL: f32 = PIXEL_PER_GRID;

#[derive(Component)]
struct SpriteGoal;
const GOAL_PIXEL: f32 = PIXEL_PER_GRID / 2.0;
const GOAL_COLOR: Color = Color::YELLOW;

#[derive(Component)]
struct SysinfoObj;
const SYSTILE_PIXEL: f32 = PIXEL_PER_GRID;
const SPRITE_DEPTH_SYSINFO: f32 =  5.0;

////////////////////////////////////////////////////////////////////////////////////////////////////

//新しい迷路を作り表示して、Playへ遷移する
fn spawn_sprite_new_map
(	mut maze: ResMut<GameMap>,
	mut state : ResMut<State<GameState>>,
	mut sysparams: ResMut<SystemParameters>,
	mut cmds: Commands,
	asset_svr: Res<AssetServer>,
)
{	//map配列を初期化する
	maze.map  .iter_mut().for_each( | x | x.fill( MapObj::Wall( None ) ) );
	maze.stat .iter_mut().for_each( | x | x.fill( BIT_ALL_CLEAR        ) );
	maze.count.iter_mut().for_each( | x | x.fill( 0                    ) );
	sysparams.stage += 1;

	//入口を掘る
	let x = maze.rng.gen_range( MAP_DIGABLE_X );
	maze.map[ x ][ MAP_HEIGHT - 2 ] = MapObj::Pathway ( None );
	maze.map[ x ][ MAP_HEIGHT - 1 ] = MapObj::DeadEnd ( None ); //入口は行き止まり扱い
	maze.start_xy = ( x, MAP_HEIGHT - 1 );

	//呼び出す関数を乱数で決め、迷路を掘らせる
	let maze_type = match sysparams.maze_type
	{	SelectMazeType::Random => maze.rng.gen_range( 0..3 ),
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
	MAP_DIGABLE_X.for_each( | x |
		if ! matches!( maze.map[ x ][ 1 ], MapObj::Wall(_) ) { exit_x.push( x ) }
	);
	let x = exit_x[ maze.rng.gen_range( 0..exit_x.len() ) ];
	maze.map[ x ][ 0 ] = MapObj::Goal( None );
	maze.goal_xy = ( x, 0 );

	//迷路の構造解析
	maze.identify_halls_and_passageways();
	maze.count_deadend_passageway_length();

	//スプライトをspawnしてEntity IDを記録する
	for x in MAP_INDEX_X
	{	for y in MAP_INDEX_Y
		{	let xy = conv_sprite_coordinates( x , y );

			let obj = &mut maze.map[ x ][ y ];
			*obj = match obj
			{	MapObj::Pathway ( o_id ) => { MapObj::Pathway ( *o_id ) }
				MapObj::DeadEnd ( o_id ) => { MapObj::Pathway ( *o_id ) }
				MapObj::Goal ( _ ) =>
				{	let id = cmds
						.spawn_bundle( sprite_goal( xy ) )
						.insert( SpriteGoal )
						.id(); 
					MapObj::Goal( Some( id ) )
				}
				MapObj::Wall ( _ ) =>
				{	let id = cmds
						.spawn_bundle( sprite_wall( xy, &asset_svr ) )
						.insert( SpriteWall { x, y } )
						.id();
					MapObj::Wall( Some( id ) )
				}
				_ => { MapObj::Space }
			};

			//行き止まり、広間のEntityを作成
			if maze.is_dead_end( x, y )
			{	let count = maze.count[ x ][ y ];
				if count > 0
				{	let info = count.to_string();
//					let id = cmds
//						.spawn_bundle ( text2d_sysinfo( xy, &asset_svr, &info ) )
//						.insert( SysinfoObj )
//						.id();
					let id = cmds
						.spawn_bundle( sprite_coin( xy, &asset_svr ) )
						.insert( SysinfoObj )
						.id();
					maze.map[ x ][ y ] = MapObj::DeadEnd( Some( id ) );
				}
			}
			else if ! maze.is_wall( x, y ) && ! maze.is_passageway( x, y )
			{	cmds.spawn_bundle( sprite_sysinfo( xy, Color::INDIGO ) )
					.insert( SysinfoObj );
			}
		}
	}

	//Playへ遷移する
	let _ = state.overwrite_set( GameState::Play );
}

//システム情報用のスプライトバンドルを生成
fn sprite_sysinfo( ( x, y ): ( f32, f32 ), color: Color ) -> SpriteBundle
{	let custom_size = Some( Vec2::new( SYSTILE_PIXEL, SYSTILE_PIXEL ) * 0.9 );

	let sprite = Sprite { color, custom_size, ..Default::default() };
	let transform = Transform::from_translation( Vec3::new( x, y, SPRITE_DEPTH_SYSINFO ) );

	SpriteBundle { sprite, transform, ..Default::default() }
}

//システム情報用のテキスト2Dバンドルを生成
fn text2d_sysinfo
(	( x, y ): ( f32, f32 ),
	asset_svr: &Res<AssetServer>,
	info: &str,
) -> Text2dBundle
{	let style = TextStyle
	{	font: asset_svr.load( FONT_MESSAGE_TEXT ),
		font_size: PIXEL_PER_GRID,
		color: Color::GRAY,
	};
	let align = TextAlignment
	{	vertical: VerticalAlign::Center,
		horizontal: HorizontalAlign::Center,
	};

	Text2dBundle
	{	text     : Text::with_section( info, style, align ),
		transform: Transform::from_translation( Vec3::new( x, y, 15.0 ) ),
		..Default::default()
	}
}

//ゴールのスプライトをアニメーションさせる
fn update_goal_sprite
(	mut q: Query<( &mut Transform, &mut Sprite ), With<SpriteGoal>>,
	time: Res<Time>,
)
{	let ( mut transform, mut sprite) = q.get_single_mut().unwrap();

	//回転させる
	let angle = 360.0 * time.delta().as_secs_f32();
	let quat = Quat::from_rotation_z( angle.to_radians() );
	transform.rotate( quat );

	//色を変える
	let hue = ( ( time.seconds_since_startup() * 500. ) as usize % 360 ) as f32;
	let ( saturation, lightness, alpha ) = ( 1., 0.5, 1. );
	sprite.color = Color::Hsla{ hue, saturation, lightness, alpha };
}

//クリアした地図の全体像を見せる
pub fn show_cleared_map
(	mut q_visibility: Query<&mut Visibility>,
	q_spr_wall_id: Query<Entity, With<SpriteWall>>,
)
{	for id in q_spr_wall_id.iter()
	{	q_visibility.get_component_mut::<Visibility>( id ).unwrap().is_visible = true;
	}
}

////////////////////////////////////////////////////////////////////////////////////////////////////

//ゴールのスプライトバンドルを生成
fn sprite_goal( ( x, y ): ( f32, f32 ) ) -> SpriteBundle
{	let position = Vec3::new( x, y, SPRITE_DEPTH_MAZE );
	let square   = Vec2::new( GOAL_PIXEL, GOAL_PIXEL );
	let quat     = Quat::from_rotation_z( 45_f32.to_radians() ); //45°傾ける
	let color    = GOAL_COLOR;

	let transform = Transform::from_translation( position ).with_rotation( quat );
	let sprite = Sprite { color, custom_size: Some( square ), ..Default::default() };

	SpriteBundle { transform, sprite, ..Default::default() }
}

//壁用のスプライトバンドルを生成
fn sprite_wall
(	( x, y ): ( f32, f32 ),
	asset_svr: &Res<AssetServer>,
) -> SpriteBundle
{	let custom_size = Some( Vec2::new( WALL_PIXEL, WALL_PIXEL ) );

	let sprite     = Sprite { custom_size, ..Default::default() };
	let texture    = asset_svr.load( WALL_SPRITE_FILE );
	let transform  = Transform::from_translation( Vec3::new( x, y, SPRITE_DEPTH_MAZE ) );

	SpriteBundle { sprite, texture, transform, ..Default::default() }
}

//コイン用のスプライトバンドルを生成
fn sprite_coin
(	( x, y ): ( f32, f32 ),
	asset_svr: &Res<AssetServer>,
) -> SpriteBundle
{	let custom_size = Some( Vec2::new( COIN_PIXEL, COIN_PIXEL ) );

	let sprite     = Sprite { custom_size, ..Default::default() };
	let texture    = asset_svr.load( COIN_SPRITE_FILE );
	let transform  = Transform::from_translation( Vec3::new( x, y, SPRITE_DEPTH_MAZE ) );

	SpriteBundle { sprite, texture, transform, ..Default::default() }
}

//End of code.