use super::*;

//external modules
#[cfg( debug_assertions )]
use bevy_inspector_egui::quick::WorldInspectorPlugin;

//submodules
mod fetch_assets;
mod spawn_text_ui;

pub use fetch_assets::*; //re-export
use spawn_text_ui::*;

//プラグインの設定
pub struct InitApp;
impl Plugin for InitApp
{   fn build( &self, app: &mut App )
    {   //メインウィンドウ、背景色、アンチエイリアシング、プラグイン
        let window = WindowDescriptor
        {   title    : APP_TITLE.to_string(),
            width    : SCREEN_PIXELS_WIDTH,
            height   : SCREEN_PIXELS_HEIGHT,
            resizable: false,
//          fit_canvas_to_parent: true, //ブラウザ表示で3Dが全画面になるのに対し2Dはそのまま拡大されず具合が悪い。2Dカメラの問題か？
            ..default()
        };
        let main_window = WindowPlugin { window, ..default() };

        app
        .insert_resource( ClearColor( SCREEN_BACKGROUND_COLOR ) )
        .insert_resource( Msaa { samples: 4 } )
        .add_plugins( DefaultPlugins.set( main_window ) )
        .add_plugin( LookTransformPlugin )          //オービットカメラ(1)
        .add_plugin( OrbitCameraPlugin::default() ) //オービットカメラ(2)
        ;

        #[cfg( debug_assertions )]
        app.add_plugin( WorldInspectorPlugin );

        //ResourceとEventを登録
        app
        .add_state( GameState::InitApp ) //Stateの初期化
        ;

        //ステージ共通のSystem
        #[cfg( not( target_arch = "wasm32" ) )]
        app.add_system( toggle_window_mode ); //[Alt]+[Enter]でフルスクリーン

        //GameState::InitApp
        //------------------------------------------------------------------------------------------
        app
        .add_plugin( FetchAssets ) //Fonts、Sprites等のプリロード
        .add_plugin( SpawnTextUi ) //Text UIのspawn
        .add_system_set
        (   SystemSet::on_exit( GameState::InitApp ) //<EXIT>
            .with_system( spawn_camera )             //カメラのspawn
            .with_system( spawn_game_frame )         //ゲームの枠の表示
        )
        ;
        //------------------------------------------------------------------------------------------
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

//bevyのカメラの設置
pub fn spawn_camera
(	mut cmds: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
)
{   use bevy::core_pipeline::clear_color::ClearColorConfig;
	cmds
	.spawn( Camera2dBundle::default() )
    .insert( Camera { priority: 1, ..default() } )
    .insert( Camera2d { clear_color: ClearColorConfig::None } ) //透過
	;
    cmds
    .spawn( Camera3dBundle::default() )
    .insert( Camera { priority: 0, ..default() } )
    .insert
    (   OrbitCameraBundle::new
        (   OrbitCameraController::default(),
            Vec3::new( -10.0, 30.0, 20.0 ),
            Vec3::new( 0.0, 0.0, 0.0 ),
        )
    );

	let light = PointLightBundle
    {   point_light: PointLight
        {   intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz( 4.0, 8.0, 4.0 ),
        ..default()
    };

    let plane = PbrBundle
    {   mesh: meshes.add( Mesh::from( shape::Plane { size: MAP_GRIDS_SHARP_PLANE as f32 } ) ),
        material: materials.add( Color::DARK_GREEN.into() ),
        ..default()
    }; 

    cmds.spawn( light );
    cmds.spawn( plane );
}

////////////////////////////////////////////////////////////////////////////////////////////////////

//ゲームの枠を表示する
fn spawn_game_frame
(   mut cmds : Commands,
    asset_svr: Res<AssetServer>,
)
{   let custom_size = Some ( ScreenPixel::new( PIXELS_PER_GRID, PIXELS_PER_GRID ) );

    for ( y, line ) in DESIGN_GAME_FRAME.iter().enumerate()
    {   for ( x, char ) in line.chars().enumerate()
        {   let pixel_xy = ScreenGrid::new( x as i32, y as i32 ).into_pixel();
            match char
            {   '#' =>
                {   cmds
                    .spawn( SpriteBundle::default() )
                    .insert( Sprite { custom_size, ..default() } )
                    .insert( Transform::from_translation( pixel_xy.extend( DEPTH_SPRITE_GAME_FRAME ) ) )
                    .insert( asset_svr.load( ASSETS_SPRITE_BRICK_WALL ) as Handle<Image> )
                    ;
                }
                '=' =>
                {   cmds
                    .spawn( SpriteBundle::default() )
                    .insert( Sprite { custom_size, color: Color::BLACK, ..default() } )
                    .insert( Transform::from_translation( pixel_xy.extend( DEPTH_SPRITE_GAME_FRAME ) ) )
                    ;
                }
                _ => ()
            }
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

//ウィンドウとフルスクリーンを切り替える
#[cfg( not( target_arch = "wasm32" ) )]
pub fn toggle_window_mode
(   inkey: Res<Input<KeyCode>>,
    inbtn: Res<Input<GamepadButton>>,
    mut window: ResMut<Windows>,
)
{   //パッドのボタンの状態
    let btn_fullscreen = GamepadButton::new( GAMEPAD, _BUTTON_FULLSCREEN );
    let is_btn_fullscreen = inbtn.just_pressed( btn_fullscreen );

    //Alt＋Enterキーの状態
    let is_key_fullscreen =
    ( inkey.pressed( _KEY_ALT_RIGHT ) || inkey.pressed( _KEY_ALT_LEFT ) )
    && inkey.just_pressed( _KEY_FULLSCREEN );

    //入力がないなら関数脱出
    if ! is_key_fullscreen && ! is_btn_fullscreen { return }

    use bevy::window::WindowMode::*;
    if let Some( window ) = window.get_primary_mut()
    {   let mode = if window.mode() == Windowed { SizedFullscreen } else { Windowed };
        window.set_mode( mode );
    }
}

//End of code.