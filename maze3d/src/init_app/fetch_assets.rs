use super::*;

//プラグインの設定
pub struct FetchAssets;
impl Plugin for FetchAssets
{   fn build( &self, app: &mut App )
    {   //GameState::Init
        //------------------------------------------------------------------------------------------
        app
        .add_system_set
        (   SystemSet::on_enter( GameState::InitApp )       //<ENTER>
            .with_system( start_fetching_assets )           //Assetのロード開始
            .with_system( spawn_sprite_now_loading )        //スプライトの生成
        )
        .add_system_set
        (   SystemSet::on_update( GameState::InitApp )      //<UPDATE>
            .with_system( change_state_after_loading )      //ロード完了か判定しState変更
            .with_system( move_sprite_now_loading )         //ローディングアニメ
        )

//      .insert_resource( MarkAfterFetchAssets ( GameState::Debug ) ) //on_exit()以降への遷移debug用

        .add_system_set
        (   SystemSet::on_exit( GameState::InitApp )        //<EXIT>
            .with_system( despawn_entity::<SpriteTile> )    //スプライトの削除
            .with_system( despawn_entity::<Camera2D> )      //カメラの削除
//          .with_system( spawn_game_frame )                //ゲームの枠の表示
        )
        ;
        //------------------------------------------------------------------------------------------
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

//マーカーResource
#[derive( Resource )]
pub struct MarkAfterFetchAssets ( pub GameState );

//ロードしたAssetsのハンドルの保存先
#[derive( Resource )]
struct LoadedAssets { preload: Vec<HandleUntyped> }

//ローディングアニメ用スプライトのComponent
#[derive( Component )] struct Camera2D;
#[derive( Component )] struct SpriteTile ( ScreenGrid );

const  DEPTH_SPRITE_TILE: f32 = 0.0;             //スプライト重なり順
const  COLOR_SPRITE_TILE: Color = Color::YELLOW; //スプライト色

//ローディングメッセージ
const DESIGN_NOWLOADING_MESSAGE: [ &str; 13 ] = 
[//   0123456789 123456789 123456789 123456789 12345
    " ##  #           #                            ", //0
    " ##  # ### #   # #    ###  #  ##  # #  #  ##  ", //1
    " # # # # # # # # #    # # # # # #   ## # #    ", //2
    " # # # # # # # # #    # # # # # # # #### # ## ", //3
    " #  ## # #  # #  #    # # ### # # # # ## #  # ", //4
    " #  ## ###  # #  #### ### # # ##  # #  #  ##  ", //5
    "",                                               //6
    " ###                      #   #           # # ", //7
    " #  # #   ###  #  ### ### # # #  #  # ### # # ", //8
    " #  # #   #   # # #   #   # # # # #    #  # # ", //9
    " ###  #   ### # # ### ### # # # # # #  #  # # ", //10
    " #    #   #   ###   # #    # #  ### #  #      ", //11
    " #    ### ### # # ### ###  # #  # # #  #  # # ", //12
];

////////////////////////////////////////////////////////////////////////////////////////////////////

//Assetsのロードを開始する
fn start_fetching_assets
(   mut cmds: Commands,
    asset_svr: Res<AssetServer>,
)
{   //Assetsのロードを開始
    let mut preload = Vec::new();
    FETCH_ASSETS.iter().for_each( | f | preload.push( asset_svr.load_untyped( *f ) ) );

    //リソースに登録して解放しないようにする
    cmds.insert_resource( LoadedAssets { preload } );
}

//ローディングアニメ用スプライトを生成する
fn spawn_sprite_now_loading
(   mut cmds: Commands,
)
{   cmds.spawn( (Camera2dBundle::default(), Camera2D ) );

    let mut rng = rand::thread_rng();
    let color = COLOR_SPRITE_TILE;
    let custom_size = Some ( ScreenPixel::new( PIXELS_PER_GRID, PIXELS_PER_GRID ) * 0.75 );
 
    for ( goal_y, line ) in DESIGN_NOWLOADING_MESSAGE.iter().enumerate()
    {   for ( goal_x, chara ) in line.chars().enumerate()
        {   if chara == ' ' { continue }    //空白文字は無視

            //スプライトの初期座標と最終座標
            let rnd_x = rng.gen_range( SCREEN_GRIDS_RANGE_X );
            let rnd_y = rng.gen_range( SCREEN_GRIDS_RANGE_Y );
            let start = ScreenGrid::new( rnd_x, rnd_y ).into_pixel();
            let goal  = ScreenGrid::new( goal_x as i32, goal_y as i32 );

            //スプライトを作成する
            cmds
            .spawn( ( SpriteBundle::default(), SpriteTile ( goal ) ) )
            .insert( Sprite { color, custom_size, ..default() } )
            .insert( Transform::from_translation( start.extend( DEPTH_SPRITE_TILE ) ) )
            ;
        } 
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

//Assetsのロードが完了したら、Stateを変更する
fn change_state_after_loading
(   assets   : Res<LoadedAssets>,
    mut state: ResMut<State<GameState>>,
    asset_svr: Res<AssetServer>,
    o_marker : Option<Res<MarkAfterFetchAssets>>,
)
{   //プリロードが完了したか？
    for handle in assets.preload.iter()
    {   use bevy::asset::LoadState::*;
        match asset_svr.get_load_state( handle )
        {   Loaded => {} //ロード完了
            Failed =>    //ロード失敗⇒パニック
            {   let mut filename = "Unknown".to_string();
                if let Some ( asset_path ) = asset_svr.get_handle_path( handle )
                {   if let Some ( s ) = asset_path.path().to_str()
                    {   filename = s.to_string();
                    }
                }
                panic!( "Can't load asset file \"{}\"", filename )
            },
            _ => return, //on_update()の中なので関数は繰り返し呼び出される
        }
    }

    //次のStateへ遷移する
    if let Some ( x ) = o_marker
    {   let _ = state.overwrite_set( x.0 );
    }
}

//スプライトを動かしてローディングアニメを見せる
fn move_sprite_now_loading
(   mut q: Query<( &mut Transform, &SpriteTile )>,
    time : Res<Time>,
)
{   let time_delta = time.delta().as_secs_f32() * 5.0;

    let mess_width  = DESIGN_NOWLOADING_MESSAGE[ 0 ].len() as f32 * PIXELS_PER_GRID;
    let mess_height = DESIGN_NOWLOADING_MESSAGE.len() as f32 * PIXELS_PER_GRID;
    let half_screen_w = SCREEN_PIXELS_WIDTH  / 2.0;
    let half_screen_h = ( SCREEN_PIXELS_HEIGHT - mess_height ) / 2.0;
    let scale = SCREEN_PIXELS_WIDTH / mess_width;

    q.for_each_mut
    (   | ( mut transform, goal_xy ) |
        {   let mut goal = goal_xy.0.into_pixel();
            goal.x = ( goal.x + half_screen_w ) * scale - half_screen_w; //横幅の調整
            goal.y = ( goal.y + half_screen_h ) * scale - half_screen_h; //縦位置の調整

            let position = &mut transform.translation;
            position.x += ( goal.x - position.x ) * time_delta;
            position.y += ( goal.y - position.y ) * time_delta;
        }
    );
}

//End of code.