use super::*;

//submodules
mod footer;

//プラグインの設定
pub struct InitApp;
impl Plugin for InitApp
{   fn build( &self, app: &mut App )
    {   //アプリの基本的な設定
        app
        .add_state::<MyState>() //Stateの初期化
        .insert_resource( Msaa::Sample4 ) //アンチエイリアス
        .add_plugins
        (   DefaultPlugins
            .set( WindowPlugin { primary_window: MAIN_WINDOW.clone(), ..default() } ) //メインウィンドウ
            .set( ImagePlugin::default_nearest() ) //ピクセルパーフェクト
        )
        .add_plugin( FrameTimeDiagnosticsPlugin ) //FPSプラグイン
        .add_systems
        (   (   spawn_cameras, //カメラをspawn
                debug::spawn_grid.run_if( DEBUG ),
                debug::spawn_obj3d.run_if( DEBUG ),
            ).on_startup()
        )
        .add_systems
        (   (   bevy::window::close_on_esc, //[ESC]キーで終了
                misc::toggle_window_mode.run_if( NOT_WASM ), //FullScreen⇔Window切換(トグル)
            )
        )
        .add_plugin( misc::NowLoading ) //Assetのプリフェッチとローディングアニメーション
        //.insert_resource( misc::AfterLoading ( MyState::TitleDemo ) ) //for test EXIT_INITAPP
        .add_systems
        (   (   spawn_game_frame,      //ゲームの枠を表示する
                footer::spawn_ui_text, //footerにtextUIをspawn
             ).in_schedule( EXIT_INITAPP )
        )
        .add_system( footer::update_fps ) //FPS表示を更新
        ;
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

//カメラをspawnする
fn spawn_cameras( mut cmds: Commands )
{   //2Dカメラ
    cmds.spawn( Camera2dBundle::default() )
    .insert( Camera   { order      : CAMERA2D_ORDER  , ..default() } )
    .insert( Camera2d { clear_color: CAMERA2D_BGCOLOR,             } )
    ;

    //3Dカメラ
    cmds.spawn( ( Camera3dBundle:: default(), MovingCamera ) )
    .insert( Camera   { order      : CAMERA3D_ORDER  , ..default() } )
    .insert( Camera3d { clear_color: CAMERA3D_BGCOLOR, ..default() } )
    .insert( CAMERA3D_TRANSFORM.looking_at( Px3d::ZERO, Px3d::Y ) ) //.looking_at()第二引数の意味が良く分からない
    ;

    //3Dライト
    cmds.spawn( PointLightBundle::default() )
    .insert( PointLight { intensity: LIGHT_BRIGHTNESS, shadows_enabled: true, ..default() } )
    .insert( LIGHT_TRANSFORM )
    ;
}

//ゲームの枠を表示する
fn spawn_game_frame
(   mut cmds : Commands,
    asset_svr: Res<AssetServer>,
)
{   let custom_size = Some ( Px2d::new( PIXELS_PER_GRID, PIXELS_PER_GRID ) );

    for ( y, line ) in DESIGN_SCREEN_FRAME.iter().enumerate()
    {   for ( x, char ) in line.chars().enumerate()
        {   if char != ' '
            {   let px2d = Grid::new( x as i32, y as i32 ).to_screen_pixel();
                let px3d = px2d.extend( DEPTH_SPRITE_GAME_FRAME );
    
                cmds.spawn( SpriteBundle::default() )
                .insert( Sprite { custom_size, ..default() } )
                .insert( Transform::from_translation( px3d ) )
                .insert( asset_svr.load( ASSETS_SPRITE_BRICK_WALL ) as Handle<Image> )
                ;
            }
        }
    }
}

//End of code.