use super::*;

//プラグインの設定
pub struct NowLoading;
impl Plugin for NowLoading
{   fn build( &self, app: &mut App )
    {   app
        .add_systems
        (   (   start_loading, //Assetのロード開始
                spawn_sprite,  //アニメ用スプライトの生成
            )
            .in_schedule( ENTER_INITAPP )
        )
        .add_systems
        (   (   is_loading_finished, //ロード完了ならState変更
                move_sprite,         //ローディングアニメ
            )
            .in_set( UPDATE_INITAPP )
        )
        .add_systems
        (   (   despawn_entity::<SpriteTile>, //スプライトの削除
            )
            .in_schedule( EXIT_INITAPP )
        )
        ;
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

//マーカーResource
#[derive( Resource )] pub struct AfterLoading ( pub MyState );

//ロードしたAssetsのハンドルの保存先
#[derive( Resource )] struct LoadedAssets { preload: Vec<HandleUntyped> }

//アニメ用スプライト
#[derive( Component )] struct SpriteTile ( Grid );
const  DEPTH_SPRITE_TILE: f32 = 900.0; //スプライト重なり順
const  COLOR_SPRITE_TILE: Color = Color::YELLOW; //スプライト色

//ローディングメッセージ
struct LoadingMessage<'a>
{   message: Vec<&'a str>,
    width  : f32,
    height : f32,
}
static NOWLOADING: Lazy<LoadingMessage> = Lazy::new
(   ||
    {   let message = vec!
        [//   0123456789 123456789 123456789 123456789 12
            " ##  #           #                            ", //0
            " ##  # ### #   # #    ###  #  ##  # #  #  ##  ", //1
            " # # # # # # # # #    # # # # # #   ## # #    ", //2
            " # # # # # # # # #    # # # # # # # #### # ## ", //3
            " #  ## # #  # #  #    # # ### # # # # ## #  # ", //4
            " #  ## ###  # #  #### ### # # ##  # #  #  ##  ", //5
            "",                                               //6
            "",                                               //7
            " ###                      #   #           # # ", //8
            " #  # #   ###  #  ### ### # # #  #  # ### # # ", //9
            " #  # #   #   # # #   #   # # # # #    #  # # ", //10
            " ###  #   ### # # ### ### # # # # # #  #  # # ", //11
            " #    #   #   ###   # #    # #  ### #  #      ", //12
            " #    ### ### # # ### ###  # #  # # #  #  # # ", //13
        ];
        let width  = message[ 0 ].len() as f32 * PIXELS_PER_GRID;
        let height = message.len() as f32 * PIXELS_PER_GRID;
        LoadingMessage { message, width, height,}    
    }
);

////////////////////////////////////////////////////////////////////////////////////////////////////

//Assetsのロードを開始する
fn start_loading
(   mut cmds: Commands,
    asset_svr: Res<AssetServer>,
)
{   //Assetsのロードを開始
    let mut preload = Vec::new();
    FETCH_ASSETS.iter().for_each( | f | preload.push( asset_svr.load_untyped( *f ) ) );

    //リソースに登録して解放しないようにする
    cmds.insert_resource( LoadedAssets { preload } );
}

//Assetsのロードが完了したら、Stateを変更する
fn is_loading_finished
(   assets   : Res<LoadedAssets>,
    mut state: ResMut<NextState<MyState>>,
    o_marker : Option<Res< AfterLoading >>,
    asset_svr: Res<AssetServer>,
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
                panic!( "Can't load asset file \"{filename}\"" )
            },
            _ => return, //UPDATE_INITAPPなので関数は繰り返し呼び出される
        }
    }

    //次のStateへ遷移する
    let Some ( x ) = o_marker else { return };
    state.set( x.0 );
}

////////////////////////////////////////////////////////////////////////////////////////////////////

//ローディングアニメ用スプライトを生成する
fn spawn_sprite
(   mut cmds: Commands,
)
{   let mut rng = rand::thread_rng();
    let color = COLOR_SPRITE_TILE;
    let custom_size = Some ( Px2d::new( PIXELS_PER_GRID, PIXELS_PER_GRID ) * 0.9 );
 
    for ( goal_y, line ) in NOWLOADING.message.iter().enumerate()
    {   for ( goal_x, chara ) in line.chars().enumerate()
        {   //空白文字は無視
            if chara == ' ' { continue }

            //スプライトの初期座標
            let rnd_x = rng.gen_range( SCREEN_GRIDS_X_RANGE );
            let rnd_y = rng.gen_range( SCREEN_GRIDS_Y_RANGE );
            let start = Grid::new( rnd_x, rnd_y ).to_screen_pixel();
            let px3d  = start.extend( DEPTH_SPRITE_TILE );

            //スプライトをspawnする
            let goal = Grid::new( goal_x as i32, goal_y as i32 );
            cmds.spawn( ( SpriteBundle::default(), SpriteTile ( goal ) ) )
            .insert( Sprite { color, custom_size, ..default() } )
            .insert( Transform::from_translation( px3d ) )
            ;
        } 
    }
}

//スプライトを動かしてローディングアニメを見せる
fn move_sprite
(   mut q: Query<( &mut Transform, &SpriteTile )>,
    time : Res<Time>,
)
{   let time_delta = time.delta().as_secs_f32() * 5.0;

    let half_screen_w = SCREEN_PIXELS_WIDTH  / 2.0;
    let half_screen_h = SCREEN_PIXELS_HEIGHT / 2.0;

    let scale = SCREEN_PIXELS_WIDTH / NOWLOADING.width; //メッセージが横方向に長いので
    let adjust_x = NOWLOADING.width  * scale / 2.0;
    let adjust_y = NOWLOADING.height * scale / 2.0;

    q.for_each_mut
    (   | ( mut transform, goal ) |
        {   //座標の調整
            let mut goal = goal.0.to_screen_pixel();
            goal.x = ( goal.x + half_screen_w ) * scale - adjust_x;
            goal.y = ( goal.y - half_screen_h ) * scale + adjust_y;

            //ゴールへ向けてスプライトを移動
            let now = &mut transform.translation;
            now.x += ( goal.x - now.x ) * time_delta;
            now.y += ( goal.y - now.y ) * time_delta;
        }
    );
}

//End of code.