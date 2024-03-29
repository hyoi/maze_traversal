use super::*;

//Pluginの手続き
pub struct PluginFetchAssets;
impl Plugin for PluginFetchAssets
{	fn build( &self, app: &mut App )
	{	app
		//------------------------------------------------------------------------------------------
		.add_system_set											// ＜GameState::Init＞
		(	SystemSet::on_enter( GameState::Init )				// ＜on_enter()＞
				.with_system( start_fetching_assets )			// Assetの事前ロード開始
				.with_system( spawn_entity_now_loading )		// ローディングアニメ用スプライトの生成
		)
		.add_system_set											// ＜GameState::Init＞
		(	SystemSet::on_update( GameState::Init )				// ＜on_update()＞
				.with_system( change_state_after_loading )		// ロード完了⇒GameState::DemoStartへ
				.with_system( move_entity_now_loading )			// ローディングアニメ
		)
		.add_system_set											// ＜GameState::Init＞
		(	SystemSet::on_exit( GameState::Init )				// ＜on_exit()＞
				.with_system( despawn_entity::<SpriteTile> )	// スプライトの削除
		)
		//------------------------------------------------------------------------------------------
		;
	}
}

////////////////////////////////////////////////////////////////////////////////////////////////////

//ロードしたAssetのハンドルの保存先
#[derive(Resource)]
struct LoadedAssets { preload: Vec<HandleUntyped> }

//ローディングメッセージ
const PRELOADING_MESSAGE_ARRAY: [ &str; 13 ] = 
[//	 0123456789 123456789 123456789 123456789 12345
	" ##  #           #                            ", //0
	" ##  # ### #   # #    ###  #  ##  # #  #  ##  ", //1
	" # # # # # # # # #    # # # # # #   ## # #    ", //2
	" # # # # # # # # #    # # # # # # # #### # ## ", //3
	" #  ## # #  # #  #    # # ### # # # # ## #  # ", //4
	" #  ## ###  # #  #### ### # # ##  # #  #  ##  ", //5
	"",												  //6
	" ###                      #   #           # # ", //7
	" #  # #   ###  #  ### ### # # #  #  # ### # # ", //8
	" #  # #   #   # # #   #   # # # # #    #  # # ", //9
	" ###  #   ### # # ### ### # # # # # #  #  # # ", //10
	" #    #   #   ###   # #    # #  ### #  #      ", //11
	" #    ### ### # # ### ###  # #  # # #  #  # # ", //12
];

//スプライト
#[derive(Component)]
struct SpriteTile ( MapGrid );
const SPRITE_PIXEL: f32   = PIXEL_PER_GRID;
const SPRITE_COLOR: Color = Color::YELLOW;
const SPRITE_DEPTH: f32   = 0.0;

////////////////////////////////////////////////////////////////////////////////////////////////////

//Assetの事前ロードを開始する
fn start_fetching_assets
(	mut cmds: Commands,
	asset_svr: Res<AssetServer>,
)
{	//Assetのロードを開始
	let mut preload = Vec::new();
	FETCH_ASSETS.iter().for_each( | f | preload.push( asset_svr.load_untyped( *f ) ) );

	//リソースに登録して解放しないようにする
	cmds.insert_resource( LoadedAssets { preload } );
}

//Assetのロードが完了したら、Stateを変更する
fn change_state_after_loading
(	mut state : ResMut<State<GameState>>,
	assets: Res<LoadedAssets>,
	asset_svr: Res<AssetServer>,
	o_dbg_ui: Option<Res<DbgOptResUI>>,
)
{	for handle in assets.preload.iter()
	{	use bevy::asset::LoadState::*;
		match asset_svr.get_load_state( handle )
		{	Loaded => {}
			Failed => panic!(),	//ロードエラー⇒パニック
			_      => return,	//on_update()なので繰り返し関数が呼び出される
		}
	}

	//次のStateへ遷移する
	if o_dbg_ui.is_some() { let _ = state.overwrite_set( GameState::Start ); }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

//ローディングアニメ用スプライトを生成する
fn spawn_entity_now_loading( mut cmds: Commands )
{	let mut rng = rand::thread_rng();

	for ( goal_y, line ) in PRELOADING_MESSAGE_ARRAY.iter().enumerate()
	{	for ( goal_x, chara ) in line.chars().enumerate()
		{	if chara == ' ' { continue }	//空白文字は無視

			//スプライトの初期座標と最終座標
			let rnd_x = rng.gen_range( 0..MAP_WIDTH  );
			let rnd_y = rng.gen_range( 0..MAP_HEIGHT );
			let start = MapGrid { x: rnd_x, y: rnd_y }.into_pixel();
			let goal  = MapGrid { x: goal_x as i32, y: goal_y as i32 };

			//スプライトを作成する
			cmds.spawn( SpriteBundle::default() )
				.insert( Sprite
				{	color: SPRITE_COLOR,
					custom_size: Some( Vec2::new( SPRITE_PIXEL, SPRITE_PIXEL ) ),
					..default()
				} )
				.insert( Transform::from_translation( Vec3::new( start.x, start.y, SPRITE_DEPTH ) ) )
				.insert( SpriteTile ( goal ) );
		} 
	}
}

//スプライトを動かしてローディングアニメを見せる
fn move_entity_now_loading
(	mut q: Query<( &mut Transform, &SpriteTile )>,
	time: Res<Time>,
)
{	let time_delta = time.delta().as_secs_f32() * 5.0;

	let half_screen_w = SCREEN_WIDTH / 2.0;
	let mess_width = PRELOADING_MESSAGE_ARRAY[ 0 ].len() as f32 * SPRITE_PIXEL;
	let scale = SCREEN_WIDTH / mess_width;

	q.for_each_mut
	(	| ( mut transform, goal_xy ) |
		{	let mut goal = goal_xy.0.into_pixel();
			goal.x = ( goal.x + half_screen_w ) * scale - half_screen_w;	//横幅の調整

			let position = &mut transform.translation;
			position.x += ( goal.x - position.x ) * time_delta;
			position.y += ( goal.y - position.y ) * time_delta;
		}
	);
}

//End of code.