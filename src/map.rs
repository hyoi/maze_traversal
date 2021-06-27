use super::*;

//Pluginの手続き
pub struct PluginMap;
impl Plugin for PluginMap
{	fn build( &self, app: &mut AppBuilder )
	{	app
		//------------------------------------------------------------------------------------------
		.init_resource::<GameStage>()							// MAP情報のResource
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
pub const MAP_WIDTH : i32 = 30;
pub const MAP_HEIGHT: i32 = 35;

//マップの座標で、掘削可能な終端（最外壁は掘れない）
use std::ops::RangeInclusive;
const DIGABLE_X: RangeInclusive<i32> = 1..= MAP_WIDTH  - 2;
const DIGABLE_Y: RangeInclusive<i32> = 1..= MAP_HEIGHT - 2;

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

//MAP情報のResource
pub struct GameStage
{	pub rng: rand::prelude::StdRng,	//再現性がある乱数を使いたいので
	pub level: usize,
	pub map: [ [ MapObj; MAP_HEIGHT as usize ]; MAP_WIDTH as usize ],
	pub count_dots: usize,
	pub start_xy: ( i32, i32 ),
	pub goal_xy : ( i32, i32 ),
}
impl Default for GameStage
{	fn default() -> Self
	{	Self
		{	rng: StdRng::seed_from_u64( rand::thread_rng().gen::<u64>() ),	//本番用
		//	rng: StdRng::seed_from_u64( 1234567890 ),	//開発用：再現性がある乱数を使いたい場合
			level: 0,
			map: [ [ MapObj::None; MAP_HEIGHT as usize ]; MAP_WIDTH as usize ],
			start_xy: ( 0, 0 ),
			goal_xy : ( 0, 0 ),
			count_dots: 0,
		}
	}
}
impl GameStage
{	pub fn enclosure( &self, x: i32, y: i32 ) -> Encloser
	{	let get_map_obj = | dx, dy |
		{	let tmp_x = x + dx;
			let tmp_y = y + dy;
			if ! ( 0..MAP_WIDTH  ).contains( &tmp_x ) 
			|| ! ( 0..MAP_HEIGHT ).contains( &tmp_y ) { return MapObj::Wall( None ) }

			self.map[ tmp_x as usize ][ tmp_y as usize ]
		};
	
		Encloser
		{	top_left     : get_map_obj( -1, -1 ),
			top_center   : get_map_obj(  0, -1 ),
			top_right    : get_map_obj(  1, -1 ),
			middle_left  : get_map_obj( -1,  0 ),
			middle_right : get_map_obj(  1,  0 ),
			bottom_left  : get_map_obj( -1,  1 ),
			bottom_center: get_map_obj(  0,  1 ),
			bottom_right : get_map_obj(  1,  1 ),
		}
	}
}

//周囲８マスのオブジェクトをまとめる型
pub struct Encloser
{	pub top_left     : MapObj,
	pub top_center   : MapObj,
	pub top_right    : MapObj,
	pub middle_left  : MapObj,
	pub middle_right : MapObj,
	pub bottom_left  : MapObj,
	pub bottom_center: MapObj,
	pub bottom_right : MapObj,
}

//マップ座標の上下左右を表す定数
const UP   : ( i32, i32 ) = (  0, -1 );
const LEFT : ( i32, i32 ) = ( -1,  0 );
const RIGHT: ( i32, i32 ) = (  1,  0 );
const DOWN : ( i32, i32 ) = (  0,  1 );

const DIRECTION: [ ( i32, i32 ); 4 ] = [ UP, LEFT, RIGHT, DOWN ];

//Component
struct GoalSprite;

//Sprite
const SPRITE_DEPTH_MAZE: f32 = 10.0;
const WALL_PIXEL: f32   = PIXEL_PER_GRID;
pub const WALL_SPRITE_FILE: &str = "sprites/wall.png";
const DOT_RAIDUS: f32   = PIXEL_PER_GRID / 14.0;
const DOT_COLOR : Color = Color::WHITE;
const GOAL_PIXEL: f32   = PIXEL_PER_GRID / 2.0;
const GOAL_COLOR: Color = Color::YELLOW;

////////////////////////////////////////////////////////////////////////////////////////////////////

//新しい迷路を作り表示して、Playへ遷移する
fn spawn_sprite_new_map
(	mut maze: ResMut<GameStage>,
	mut state : ResMut<State<GameState>>,
	mut cmds: Commands,
	mut color_matl: ResMut<Assets<ColorMaterial>>,
	asset_svr: Res<AssetServer>,
)
{	//mapを初期化する
	maze.map.iter_mut().for_each( |x| (*x).fill( MapObj::Wall( None ) ) );
	maze.count_dots = 0;
	maze.level += 1;

	//入口を掘る
	let x = maze.rng.gen_range( DIGABLE_X );
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
	{	if DIGABLE_X.contains( &( x as i32 ) )
		&& ! matches!( ary[ 1 ], MapObj::Wall(_) ) { exit_x.push( x ) }
	}
	let x = exit_x[ maze.rng.gen_range( 0..exit_x.len() ) ];
	maze.map[ x ][ 0 ] = MapObj::Goal( None );
	maze.goal_xy = ( x as i32, 0 );

	//スプライトをspawnしてEntity IDを記録する
	let mut count = 0;
	for ( x, ary ) in maze.map.iter_mut().enumerate()
	{	for ( y, obj ) in ary.iter_mut().enumerate()
		{	let xy = conv_sprite_coordinates( x as i32, y as i32);
			match obj
			{	MapObj::Dot1(_) =>
				{	let id = cmds
						.spawn_bundle( sprite_dot( xy ) )
						.id(); 
					*obj = MapObj::Dot1( Some( id ) );
					count += 1;
				}
				MapObj::Dot2(_) =>
				{	let id = cmds
						.spawn_bundle( sprite_dot( xy ) )
						.id(); 
					*obj = MapObj::Dot1( Some( id ) ); //Dot2もDot1へ変換する
					count += 1;
				}
				MapObj::Goal(_) =>
				{	let id = cmds
						.spawn_bundle( sprite_goal( xy, &mut color_matl ) )
						.insert( GoalSprite )
						.id(); 
					*obj = MapObj::Dot1( Some( id ) );	//GoalはDot1へ変換する
					count += 1;
				}
				MapObj::Wall(_) =>
				{	let id = cmds
						.spawn_bundle( sprite_wall( xy, &mut color_matl, &asset_svr ) )
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
	let angle = 360. * time.delta().as_secs_f32();
	let quat  = Quat::from_rotation_z( angle.to_radians() );
	transform.rotate( quat );

	//色を変える
	let hue = ( ( time.seconds_since_startup() * 500. ) as usize % 360 ) as f32;
	let ( saturation, lightness, alpha ) = ( 1., 0.5, 1. );
	color_matl.color = Color::Hsla{ hue, saturation, lightness, alpha };
}

//スプライトを削除する
fn despawn_sprite_map( maze: Res<GameStage>, mut cmds: Commands )
{	for ary in maze.map.iter()
	{	for obj in ary.iter()
		{	match obj
			{	MapObj::Dot1( opt_entity ) => cmds.entity( opt_entity.unwrap() ).despawn(),
				MapObj::Wall( opt_entity ) => cmds.entity( opt_entity.unwrap() ).despawn(),
				_ => {}
			}
		}
	}
}

////////////////////////////////////////////////////////////////////////////////////////////////////

//ドット用のスプライトバンドルを生成
pub fn sprite_dot( ( x, y ): ( f32, f32 ) ) -> ShapeBundle
{	GeometryBuilder::build_as
	(	&shapes::Circle { radius: DOT_RAIDUS, ..shapes::Circle::default() },
		ShapeColors::new( DOT_COLOR ),
        DrawMode::Fill( FillOptions::default() ),
        Transform::from_translation( Vec3::new( x, y, SPRITE_DEPTH_MAZE ) ),
    )
}

//ゴールのスプライトバンドルを生成
pub fn sprite_goal( ( x, y ): ( f32, f32 ), color_matl: &mut ResMut<Assets<ColorMaterial>> ) -> SpriteBundle
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
pub fn sprite_wall
(	( x, y ): ( f32, f32 ),
	color_matl: &mut ResMut<Assets<ColorMaterial>>,
	asset_svr: &Res<AssetServer>,
) -> SpriteBundle
{	SpriteBundle
	{	material : color_matl.add( asset_svr.load( WALL_SPRITE_FILE ).into() ),
		transform: Transform::from_translation( Vec3::new( x, y, SPRITE_DEPTH_MAZE ) ),
		sprite   : Sprite::new( Vec2::new( WALL_PIXEL, WALL_PIXEL ) ),
		..Default::default()
	}
}

////////////////////////////////////////////////////////////////////////////////////////////////////

//一型迷路：ランダムに掘り進み、貫通する壁は確率で破壊する
fn dig_and_dig_and_dig( maze: &mut GameStage )
{	let mut map_xy = maze.start_xy;
	map_xy.1 -= 1; //maze.start_xyの直上(y-1)がトンネル掘りの開始座標

	loop
	{	//ランダムに上下左右へ進む方向を決める
		let ( dx, dy ) = DIRECTION[ maze.rng.gen_range( 0..DIRECTION.len() ) ];
		let tmp_x = map_xy.0 + dx;
		let tmp_y = map_xy.1 + dy;

		//上端に達したら迷路完成
		if tmp_y == 0 { break }

		//掘れないならループ先頭に戻る
		if ! DIGABLE_X.contains( &( tmp_x ) )
		|| ! DIGABLE_Y.contains( &( tmp_y ) )
		|| ! is_dig_or_not( maze, tmp_x, tmp_y ) { continue }

		//一歩進む
		maze.map[ tmp_x as usize ][ tmp_y as usize ] = MapObj::Dot1( None );
		map_xy = ( tmp_x, tmp_y );
	}
}

//さいころを振って、進むか(true)、やり直すか(false)決める
fn is_dig_or_not( maze: &mut GameStage, x: i32, y: i32 ) -> bool
{	//そもそも壁じゃないならtrue
	if ! matches!( maze.map[ x as usize ][ y as usize ], MapObj::Wall(_) ) { return true }

	//座標の周囲のオブジェクトを取り出す
	let objs = maze.enclosure( x, y );

	//上下左右のオブジェクトで壁ではないものを数える
	let mut count = 0;
	if ! matches!( objs.top_center   , MapObj::Wall(_) ) { count += 1 }
	if ! matches!( objs.middle_left  , MapObj::Wall(_) ) { count += 1 }
	if ! matches!( objs.middle_right , MapObj::Wall(_) ) { count += 1 }
	if ! matches!( objs.bottom_center, MapObj::Wall(_) ) { count += 1 }

	//２以上なら掘ると道になるので、貫通させるか確率で決める
	let dice = maze.rng.gen_range( 0..100 );	//百面ダイスを振って‥‥
	if count == 2 && dice < 70 { return false }	//通路になる   ⇒ 70%の確率でfalse
	if count == 3 && dice < 90 { return false }	//Ｔ字路になる ⇒ 90%の確率でfalse
	if count == 4 && dice < 95 { return false }	//十字路になる ⇒ 95%の確率でfalse

	//壁を掘り進む
	true
}

////////////////////////////////////////////////////////////////////////////////////////////////////

//二型迷路：ランダムに掘り進み、貫通せず、行き止まりでは後戻りして掘り尽くすまで未掘削の場所を探す
fn dig_and_back_and_dig( maze: &mut GameStage )
{	let mut map_xy = maze.start_xy;
	map_xy.1 -= 1; //maze.start_xyの直上(y-1)がトンネル掘りの開始座標

	//トンネルを掘る
	let mut digable_walls = Vec::new();
	let mut backtrack;
	loop
	{	//上下左右にある掘削候補と戻り路を記録する
		digable_walls.clear();
		backtrack = ( 0, 0 );
		for ( dx, dy ) in DIRECTION.iter()
		{	let tmp_x = map_xy.0 + dx;
			let tmp_y = map_xy.1 + dy;
			let tmp_xy = ( tmp_x, tmp_y ); 

			//外壁は掘れない
			if ! DIGABLE_X.contains( &tmp_x ) || ! DIGABLE_Y.contains( &tmp_y ) { continue }
	
			//上下左右の座標のオブジェクトを調べる
			match maze.map[ tmp_x as usize ][ tmp_y as usize ]
			{	MapObj::Dot1(_)
					=> backtrack = tmp_xy,
				MapObj::Wall(_) if is_digable_wall( maze, tmp_xy, ( *dx, *dy ) )
					=> digable_walls.push( tmp_xy ),
				_	=> {}
			}
		}

		//掘れる壁が見つからないなら迷路完成
		if digable_walls.is_empty() && backtrack == ( 0, 0 ) { break }

		if ! digable_walls.is_empty()
		{	//掘削する方向をランダムに決めて、掘る
			map_xy = digable_walls[ maze.rng.gen_range( 0..digable_walls.len() ) ];
			maze.map[ map_xy.0 as usize ][ map_xy.1 as usize ] = MapObj::Dot1( None );
		}
		else
		{	//掘れる壁がないので現在位置に行き止まり情報「dot2」を書き込み、後戻りする
			maze.map[ map_xy.0 as usize ][ map_xy.1 as usize ] = MapObj::Dot2( None );
			map_xy = backtrack;
		}
	}

	//三型迷路の作成関数を流用して、道幅拡張工事
	find_and_destroy_digable_walls( maze );
} 

//進行方向の壁が掘れるか調べる
fn is_digable_wall( maze: &GameStage, ( x, y ): ( i32, i32 ), direction: ( i32, i32 ) ) -> bool
{	let objs = maze.enclosure( x, y );
	match direction
	{	UP    if matches!( objs.top_left     , MapObj::Wall(_) )
			  && matches!( objs.top_center   , MapObj::Wall(_) )	// 壁壁壁
			  && matches!( objs.top_right    , MapObj::Wall(_) )	// 壁Ｘ壁
			  && matches!( objs.middle_left  , MapObj::Wall(_) )
			  && matches!( objs.middle_right , MapObj::Wall(_) ) => return true,
		LEFT  if matches!( objs.top_left     , MapObj::Wall(_) )	// 壁壁
			  && matches!( objs.top_center   , MapObj::Wall(_) )	// 壁Ｘ
			  && matches!( objs.middle_left  , MapObj::Wall(_) )	// 壁壁
			  && matches!( objs.bottom_left  , MapObj::Wall(_) )
			  && matches!( objs.bottom_center, MapObj::Wall(_) ) => return true,
		RIGHT if matches!( objs.top_center   , MapObj::Wall(_) )	// 壁壁
			  && matches!( objs.top_right    , MapObj::Wall(_) )	// Ｘ壁
			  && matches!( objs.middle_right , MapObj::Wall(_) )	// 壁壁
			  && matches!( objs.bottom_center, MapObj::Wall(_) )
			  && matches!( objs.bottom_right , MapObj::Wall(_) ) => return true,
		DOWN  if matches!( objs.middle_left  , MapObj::Wall(_) )
			  && matches!( objs.middle_right , MapObj::Wall(_) )	// 壁Ｘ壁
			  && matches!( objs.bottom_left  , MapObj::Wall(_) )	// 壁壁壁
			  && matches!( objs.bottom_center, MapObj::Wall(_) )
			  && matches!( objs.bottom_right , MapObj::Wall(_) ) => return true,
		_ => {}
	}

	false
}

////////////////////////////////////////////////////////////////////////////////////////////////////

//三型迷路：マップを全面走査して、壊すと迷路を拡張可能な壁を探し、壊し尽くすまで壊しまくる
fn find_and_destroy_digable_walls( maze: &mut GameStage )
{	let mut digable_walls = Vec::new();
	loop
	{	//マップを全面走査して拡張条件を満たす壁を探す
		digable_walls.clear();
		for ( x, ary ) in maze.map.iter().enumerate()	//xはusize
		{	for ( y, _obj ) in ary.iter().enumerate()	//yはusize
			{	if ! DIGABLE_X.contains( &( x as i32 ) )
				|| ! DIGABLE_Y.contains( &( y as i32 ) )
				|| ! matches!( maze.map[ x ][ y ], MapObj::Wall(_) ) { continue }

				//条件を満たす壁を記録する
				if is_maze_expandable( maze, x, y ) { digable_walls.push( ( x, y ) ) }
			}
		}

		//条件を満たす壁が見つからなければ迷路完成
		if digable_walls.is_empty() { break }

		//複数候補の中からランダムに壊す壁を決め、道にする
		let ( x, y ) = digable_walls[ maze.rng.gen_range( 0..digable_walls.len() ) ];
		maze.map[ x ][ y ] = MapObj::Dot1( None );
	}
}

//迷路拡張条件を満たす壁か？
fn is_maze_expandable( maze: &GameStage, x:usize, y:usize ) -> bool
{	let objs = maze.enclosure( x as i32, y as i32 );

	//下向き凸の削り許可
	if   matches!( objs.top_left     , MapObj::Wall(_) ) &&
	     matches!( objs.top_center   , MapObj::Wall(_) ) &&
	     matches!( objs.top_right    , MapObj::Wall(_) ) &&
	   ! matches!( objs.middle_left  , MapObj::Wall(_) ) &&
	   ! matches!( objs.middle_right , MapObj::Wall(_) ) &&
	   ! matches!( objs.bottom_left  , MapObj::Wall(_) ) &&
	   ! matches!( objs.bottom_center, MapObj::Wall(_) ) &&
	   ! matches!( objs.bottom_right , MapObj::Wall(_) ) { return true }

	//右向き凸の削り許可
	if   matches!( objs.top_left     , MapObj::Wall(_) ) &&
	   ! matches!( objs.top_center   , MapObj::Wall(_) ) &&
	   ! matches!( objs.top_right    , MapObj::Wall(_) ) &&
	     matches!( objs.middle_left  , MapObj::Wall(_) ) &&
	   ! matches!( objs.middle_right , MapObj::Wall(_) ) &&
	     matches!( objs.bottom_left  , MapObj::Wall(_) ) &&
	   ! matches!( objs.bottom_center, MapObj::Wall(_) ) &&
	   ! matches!( objs.bottom_right , MapObj::Wall(_) ) { return true }

	//左向き凸の削り許可
	if ! matches!( objs.top_left     , MapObj::Wall(_) ) &&
	   ! matches!( objs.top_center   , MapObj::Wall(_) ) &&
	     matches!( objs.top_right    , MapObj::Wall(_) ) &&
	   ! matches!( objs.middle_left  , MapObj::Wall(_) ) &&
	     matches!( objs.middle_right , MapObj::Wall(_) ) &&
	   ! matches!( objs.bottom_left  , MapObj::Wall(_) ) &&
	   ! matches!( objs.bottom_center, MapObj::Wall(_) ) &&
	     matches!( objs.bottom_right , MapObj::Wall(_) ) { return true }

	//上向き凸の削り許可
	if ! matches!( objs.top_left     , MapObj::Wall(_) ) &&
	   ! matches!( objs.top_center   , MapObj::Wall(_) ) &&
	   ! matches!( objs.top_right    , MapObj::Wall(_) ) &&
	   ! matches!( objs.middle_left  , MapObj::Wall(_) ) &&
	   ! matches!( objs.middle_right , MapObj::Wall(_) ) &&
	     matches!( objs.bottom_left  , MapObj::Wall(_) ) &&
	     matches!( objs.bottom_center, MapObj::Wall(_) ) &&
	     matches!( objs.bottom_right , MapObj::Wall(_) ) { return true }

	//縦の貫通路になる場合はfalse
	if ! matches!( objs.top_center   , MapObj::Wall(_) ) &&
	   ! matches!( objs.bottom_center, MapObj::Wall(_) ) { return false }

	//横の貫通路になる場合はfalse
	if ! matches!( objs.middle_left , MapObj::Wall(_) ) &&
	   ! matches!( objs.middle_right, MapObj::Wall(_) ) { return false }

	//左上が壁でなく、上と左が壁ならfalse
	if ! matches!( objs.top_left   , MapObj::Wall(_) ) &&
		 matches!( objs.top_center , MapObj::Wall(_) ) &&
		 matches!( objs.middle_left, MapObj::Wall(_) ) { return false }

	//右上が壁でなく、上と右が壁ならfalse
	if ! matches!( objs.top_right   , MapObj::Wall(_) ) &&
		 matches!( objs.top_center  , MapObj::Wall(_) ) &&
		 matches!( objs.middle_right, MapObj::Wall(_) ) { return false }

	//左下が壁でなく、下と左が壁ならfalse
	if ! matches!( objs.bottom_left  , MapObj::Wall(_) ) &&
		 matches!( objs.middle_left  , MapObj::Wall(_) ) &&
		 matches!( objs.bottom_center, MapObj::Wall(_) ) { return false }

	//右下が壁でなく、下と右が壁ならfalse
	if ! matches!( objs.bottom_right , MapObj::Wall(_) ) &&
		 matches!( objs.middle_right , MapObj::Wall(_) ) &&
		 matches!( objs.bottom_center, MapObj::Wall(_) ) { return false }

	//上下左右がすべて壁はfalse（掘ると飛び地になる）
	if matches!( objs.top_center   , MapObj::Wall(_) ) &&
	   matches!( objs.middle_left  , MapObj::Wall(_) ) &&
	   matches!( objs.middle_right , MapObj::Wall(_) ) &&
	   matches!( objs.bottom_center, MapObj::Wall(_) ) { return false }

	//掘削できる壁
	true
}

//End of code.