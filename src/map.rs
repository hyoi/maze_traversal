use super::*;

//リソース関係
#[derive(Copy,Clone,PartialEq)]
pub enum MapObj
{	None,
	Wall ( Option<Entity> ),
	Dot1 ( Option<Entity> ),	//通常の道
	Dot2 ( Option<Entity> ),	//行き止まり目印用
	Goal ( Option<Entity> ),
	Space,
}
impl Default for GameStage
{	fn default() -> Self
	{	Self
		{	rng: StdRng::seed_from_u64( rand::thread_rng().gen::<u64>() ),	//本番用
		//	rng: StdRng::seed_from_u64( 1234567890 ),	//開発用：再現性がある乱数を使いたい場合
			level: 0,
			map: [ [ MapObj::None; MAP_HEIGHT ]; MAP_WIDTH ],
			start_xy: ( 0, 0 ),
			goal_xy : ( 0, 0 ),
			count_dots: 0,
		}
	}
}

////////////////////////////////////////////////////////////////////////////////

//マップ座標の上下左右を表す定数
pub const UP   : ( i32, i32 ) = (  0, -1 );
pub const LEFT : ( i32, i32 ) = ( -1,  0 );
pub const RIGHT: ( i32, i32 ) = (  1,  0 );
pub const DOWN : ( i32, i32 ) = (  0,  1 );
const DIRECTION: [ ( i32, i32 ); 4 ] = [ UP, LEFT, RIGHT, DOWN ];

//マップの掘れる範囲
pub const MAX_X: usize = MAP_WIDTH  - 2; //1..=MAX_X
pub const MAX_Y: usize = MAP_HEIGHT - 2; //1..=MAX_Y

////////////////////////////////////////////////////////////////////////////////

//新しい迷路を作り、スプライトを配置する
pub fn spawn_sprite_new_map
(	mut stage: ResMut<GameStage>,
	mut cmds: Commands,
	mut color_matl: ResMut<Assets<ColorMaterial>>,
	asset_svr: Res<AssetServer>,
)
{	//mapを初期化する
	stage.map.iter_mut().for_each( |x| (*x).fill( MapObj::Wall( None ) ) );
	stage.count_dots = 0;
	stage.level += 1;

	//入口を掘る
	let x = stage.rng.gen_range( 1..=MAX_X );
	stage.map[ x ][ MAX_Y     ] = MapObj::Dot1( None );
	stage.map[ x ][ MAX_Y + 1 ] = MapObj::Dot2( None ); //出口は行き止まり扱い
	stage.start_xy = ( x, MAX_Y + 1 );

	//トンネルを掘る（各関数に任せる）
	match stage.rng.gen_range( 0..3 )
	{	0 => dig_and_dig_and_dig( &mut stage ),
		1 => dig_and_back_and_dig( &mut stage ), 
		2 => find_and_destroy_digable_walls( &mut stage ),
		_ => {}
	}

	//出口を掘れる場所を探す
	let mut exit_x = Vec::new();
	for ( x, ary ) in stage.map.iter().enumerate()
	{	if ! ( 1..=MAX_X ).contains( &x ) { continue }
		if ! matches!( ary[ 1 ], MapObj::Wall(_) ) { exit_x.push( x ) }
	}

	//出口を掘る
	let x = exit_x[ stage.rng.gen_range( 0..exit_x.len() ) ];
	stage.map[ x ][ 0 ] = MapObj::Goal( None );
	stage.goal_xy = ( x, 0 );

	//スプライトをspawnしてEntity IDを記録する
	let mut count = 0;
	for ( x, ary ) in stage.map.iter_mut().enumerate()
	{	for ( y, obj ) in ary.iter_mut().enumerate()
		{	let xy = conv_sprite_coordinates( x, y );
			if matches!( obj, MapObj::Dot1(_) ) || matches!( obj, MapObj::Dot2(_) )
			{	let id = cmds.spawn_bundle( sprite_dot( xy ) ).id(); 
				*obj = MapObj::Dot1( Some( id ) ); //Dot2もDot1へ変換する
				count += 1;
			}
			else if matches!( obj, MapObj::Goal(_) )
			{	let id = cmds.spawn_bundle( sprite_goal( xy, &mut color_matl ) ).insert( GoalSprite ).id(); 
				*obj = MapObj::Dot1( Some( id ) );	//GoalはDot1へ変換する
				count += 1;
			}
			else if matches!( obj, MapObj::Wall(_) )
			{	let id = cmds.spawn_bundle( sprite_wall( xy, &mut color_matl, &asset_svr ) ).id();
				*obj = MapObj::Wall( Some( id ) );
			}
			else
			{	*obj = MapObj::Space;
			}
		}
	}
	stage.count_dots = count;
}

////////////////////////////////////////////////////////////////////////////////

pub struct GoalSprite;

//ゴールのスプライトをアニメーションさせる
pub fn animate_goal_sprite
(	mut q_goal: Query<( &mut Transform, &Handle<ColorMaterial> ), With<GoalSprite>>,
	mut assets_color_matl: ResMut<Assets<ColorMaterial>>,
	time: Res<Time>,
)
{	let ( mut transform, handle ) = q_goal.single_mut().unwrap();
	let color_matl = assets_color_matl.get_mut( handle ).unwrap();

	//回転させる
	let angle = 360. * time.delta().as_secs_f32();
	let quat  = Quat::from_rotation_z( angle.to_radians() );
	transform.rotate( quat );

	//色を変える
	let angle = ( ( time.seconds_since_startup() * 500. ) as usize % 360 ) as f32;
	let ( saturation, lightness, alpha ) = ( 1., 0.5, 1. );
	color_matl.color = Color::Hsla{ hue: angle, saturation, lightness, alpha };
}

////////////////////////////////////////////////////////////////////////////////

//スプライトを削除する
pub fn despawn_sprite_map( stage: Res<GameStage>, mut cmds: Commands )
{	for ary in stage.map.iter()
	{	for obj in ary.iter()
		{	if let MapObj::Dot1( opt_entity ) = obj
			{	cmds.entity( opt_entity.unwrap() ).despawn();
			}
			if let MapObj::Wall( opt_entity ) = obj
			{	cmds.entity( opt_entity.unwrap() ).despawn();
			}
		}
	}
}

////////////////////////////////////////////////////////////////////////////////

//迷路一型：ランダム進行＋確率で壁を破壊し貫通させる
fn dig_and_dig_and_dig( stage: &mut GameStage )
{	let mut map_xy = stage.start_xy;
	map_xy.1 -= 1; //stage.start_xyの直上(y-1)がトンネル掘りの開始座標

	loop
	{	//ランダムに上下左右へ一歩進んだ座標を作り、
		//座標がマップからはみ出していたらやり直し（tmp_y==0はゴールなので例外扱い）
		let ( dx, dy ) = DIRECTION[ stage.rng.gen_range( 0..DIRECTION.len() ) ];
		let tmp_x = ( map_xy.0 as i32 + dx ) as usize;
		let tmp_y = ( map_xy.1 as i32 + dy ) as usize;
		if ! ( 1..=MAX_X ).contains( &tmp_x ) || tmp_y > MAX_Y { continue }

		//さいころを振って、進むか(true)、やり直すか(false)決める
		let tmp_xy = ( tmp_x, tmp_y ); 
		if ! whether_dig_or_not( stage, tmp_xy ) { continue }

		//上端に達したら完成
		if tmp_y == 0 { break }

		//一歩進む
		stage.map[ tmp_x ][ tmp_y ] = MapObj::Dot1( None );
		map_xy = tmp_xy;
	}
}

//さいころを振って、進むか(true)、やり直すか(false)決める
fn whether_dig_or_not( stage: &mut GameStage, ( x, y ): ( usize, usize ) ) -> bool
{	if y == 0 { return true } //ゴールは無条件にtrue
	if ! matches!( stage.map[ x ][ y ], MapObj::Wall(_) ) { return true } //そもそも壁じゃないならtrue

	//座標の周囲のオブジェクトを取り出す
	let objs = get_enclosing_objects( stage, ( x, y ) );

	//上下左右のオブジェクトで壁ではないものを数える
	let mut count = 0;
	if ! matches!( objs.top_center   , MapObj::Wall(_) ) { count += 1 }
	if ! matches!( objs.middle_left  , MapObj::Wall(_) ) { count += 1 }
	if ! matches!( objs.middle_right , MapObj::Wall(_) ) { count += 1 }
	if ! matches!( objs.bottom_center, MapObj::Wall(_) ) { count += 1 }

	//２以上なら掘ると道になるので、貫通させるか確率で決める
	let dice = stage.rng.gen_range( 0..100 );	//百面ダイスを振って‥‥
	if count == 2 && dice < 70 { return false }	//通路になる   ⇒ 70%の確率でfalse
	if count == 3 && dice < 90 { return false }	//Ｔ字路になる ⇒ 90%の確率でfalse
	if count == 4 && dice < 95 { return false }	//十字路になる ⇒ 95%の確率でfalse

	//壁を掘り進む
	true
}

////////////////////////////////////////////////////////////////////////////////

//迷路二型：ランダム進行＋貫通不可＋後戻りして掘れる場所から掘り続ける
fn dig_and_back_and_dig( stage: &mut GameStage )
{	let mut map_xy = stage.start_xy;
	map_xy.1 -= 1; //stage.start_xyの直上(y-1)がトンネル掘りの開始座標

	//トンネルを掘る
	let mut digable_walls = Vec::new();
	let mut backtrack;
	loop
	{	//上下左右にある掘削候補と戻り路を記録する
		digable_walls.clear();
		backtrack = ( 0, 0 );
		for dxdy in DIRECTION.iter()
		{	let ( dx, dy ) = *dxdy;
			let tmp_x = ( map_xy.0 as i32 + dx ) as usize;
			let tmp_y = ( map_xy.1 as i32 + dy ) as usize;
			let tmp_xy = ( tmp_x, tmp_y ); 
	
			//上下左右の座標のオブジェクトを調べる
			match stage.map[ tmp_x ][ tmp_y ]
			{	MapObj::Dot1(_)
					=> backtrack = tmp_xy,
				MapObj::Wall(_) if is_digable_wall( stage, tmp_xy, *dxdy )
					=> digable_walls.push( tmp_xy ),
				_	=> {}
			}
		}
	
		//掘れる壁が見つからないなら完成
		if digable_walls.is_empty() && backtrack == ( 0, 0 ) { break }

		if ! digable_walls.is_empty()
		{	//掘削する方向をランダムに決めて、掘る
			map_xy = digable_walls[ stage.rng.gen_range( 0..digable_walls.len() ) ];
			stage.map[ map_xy.0 ][ map_xy.1 ] = MapObj::Dot1( None );
		}
		else
		{	//掘れる壁がないので現在位置に行き止まり情報「dot2」を書き込み、後戻りする
			stage.map[ map_xy.0 ][ map_xy.1 ] = MapObj::Dot2( None );
			map_xy = backtrack;
		}
	}

	//アルゴリズム三型を流用して道幅を拡張する
	find_and_destroy_digable_walls( stage );
} 

//進行方向の壁が掘れるか調べる
fn is_digable_wall( stage: &GameStage, ( x, y ): ( usize, usize ), direction: ( i32, i32 ) ) -> bool
{	if direction == UP
	{	if ! ( 1..=MAX_X ).contains( &x ) || y < 1 { return false }
		let objs = get_enclosing_objects( stage, ( x, y ) );
		if matches!( objs.top_left    , MapObj::Wall(_) ) &&	// 
		   matches!( objs.top_center  , MapObj::Wall(_) ) &&	// 壁壁壁
		   matches!( objs.top_right   , MapObj::Wall(_) ) &&	// 壁Ｘ壁
		   matches!( objs.middle_left , MapObj::Wall(_) ) &&	//
		   matches!( objs.middle_right, MapObj::Wall(_) ) { return true }
	}
	else if direction == LEFT
	{	if x < 1 || ! ( 1..=MAX_Y ).contains( &y ) { return false }
		let objs = get_enclosing_objects( stage, ( x, y ) );
		if matches!( objs.top_left     , MapObj::Wall(_) ) &&	// 壁壁
		   matches!( objs.top_center   , MapObj::Wall(_) ) &&	// 壁Ｘ
		   matches!( objs.middle_left  , MapObj::Wall(_) ) &&	// 壁壁
		   matches!( objs.bottom_left  , MapObj::Wall(_) ) &&	//
		   matches!( objs.bottom_center, MapObj::Wall(_) ) { return true }
	}
	else if direction == RIGHT
	{	if x > MAX_X || ! ( 1..=MAX_Y ).contains( &y ) { return false }
		let objs = get_enclosing_objects( stage, ( x, y ) );
		if matches!( objs.top_center   , MapObj::Wall(_) ) &&	// 壁壁
		   matches!( objs.top_right    , MapObj::Wall(_) ) &&	// Ｘ壁
		   matches!( objs.middle_right , MapObj::Wall(_) ) &&	// 壁壁
		   matches!( objs.bottom_center, MapObj::Wall(_) ) &&	// 
		   matches!( objs.bottom_right , MapObj::Wall(_) ) { return true }
	}
	else if direction == DOWN
	{	if ! ( 1..=MAX_X ).contains( &x ) || y > MAX_Y { return false }
		let objs = get_enclosing_objects( stage, ( x, y ) );
		if matches!( objs.middle_left  , MapObj::Wall(_) ) &&	// 
		   matches!( objs.middle_right , MapObj::Wall(_) ) &&	// 壁Ｘ壁
		   matches!( objs.bottom_left  , MapObj::Wall(_) ) &&	// 壁壁壁
		   matches!( objs.bottom_center, MapObj::Wall(_) ) &&	// 
		   matches!( objs.bottom_right , MapObj::Wall(_) ) { return true }
	}

	false
}

////////////////////////////////////////////////////////////////////////////////

//迷路三型：マップを全面走査して、壊すと迷路を拡張可能な壁を探し、壊しまくる
fn find_and_destroy_digable_walls( stage: &mut GameStage )
{	let mut digable_walls = Vec::new();
	loop
	{	//マップを全面走査して拡張条件を満たす壁をすべて記録する
		digable_walls.clear();
		for ( x, ary ) in stage.map.iter().enumerate()
		{	for ( y, _obj ) in ary.iter().enumerate()
			{	//処理スキップ
				if ! matches!( stage.map[ x ][ y ], MapObj::Wall(_) ) { continue }
				if ! ( ( 1..=MAX_X ).contains( &x ) && ( 1..=MAX_Y ).contains( &y ) ) { continue }

				//迷路拡張条件を満たす壁か？
				if ! is_maze_expandable( stage, ( x, y ) ) { continue }

				digable_walls.push( ( x, y ) );
			}
		}

		//条件を満たす壁が見つからなければ完成
		if digable_walls.is_empty() { break }

		//複数候補の中からランダムに壊す壁を決め、道にする
		let ( x, y ) = digable_walls[ stage.rng.gen_range( 0..digable_walls.len() ) ];
		stage.map[ x ][ y ] = MapObj::Dot1( None );
	}
}

//迷路拡張条件を満たす壁か？
fn is_maze_expandable( stage: &GameStage, map_xy: ( usize, usize ) ) -> bool
{	//中心座標の周囲８マスのオブジェクトを取り出す
	let objs = get_enclosing_objects( stage, map_xy );

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

////////////////////////////////////////////////////////////////////////////////

//周囲８マスのオブジェクトをまとめる型
struct Encloser
{	top_left     : MapObj,
	top_center   : MapObj,
	top_right    : MapObj,
	middle_left  : MapObj,
	middle_right : MapObj,
	bottom_left  : MapObj,
	bottom_center: MapObj,
	bottom_right : MapObj,
}

//中心座標の周囲８マスのオブジェクトを返す。配列からはみ出した箇所は壁扱い
fn get_enclosing_objects( stage: &GameStage, ( x, y ): ( usize, usize ) ) -> Encloser
{	let get_map_obj = | dx, dy | { stage.map[( x as i32 + dx ) as usize][( y as i32 + dy ) as usize] };

	Encloser
	{	top_left  : if x >= 1     && y >= 1 { get_map_obj( -1, -1 ) } else { MapObj::Wall( None ) },
		top_center: if               y >= 1 { get_map_obj(  0, -1 ) } else { MapObj::Wall( None ) },
		top_right : if x <= MAX_X && y >= 1 { get_map_obj(  1, -1 ) } else { MapObj::Wall( None ) },

		middle_left : if x >= 1     { get_map_obj( -1, 0 ) } else { MapObj::Wall( None ) },
		middle_right: if x <= MAX_X { get_map_obj(  1, 0 ) } else { MapObj::Wall( None ) },

		bottom_left  : if x >= 1     && y <= MAX_Y { get_map_obj( -1, 1 ) } else { MapObj::Wall( None ) },
		bottom_center: if               y <= MAX_Y { get_map_obj(  0, 1 ) } else { MapObj::Wall( None ) },
		bottom_right : if x <= MAX_X && y <= MAX_Y { get_map_obj(  1, 1 ) } else { MapObj::Wall( None ) },
	}
}

//End of code.