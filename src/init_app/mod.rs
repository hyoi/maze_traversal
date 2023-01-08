use super::*;

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
            fit_canvas_to_parent: true,
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

        //ResourceとEventを登録
        app
        .add_state( GameState::InitApp ) //Stateの初期化
        ;

        //GameState::InitApp
        //------------------------------------------------------------------------------------------
        app
        .add_plugin( FetchAssets ) //Fonts、Sprites等のプリロード
        .add_plugin( SpawnTextUi ) //Text UIのspawn
        .add_system_set
        (   SystemSet::on_exit( GameState::InitApp )    //<EXIT>
            .with_system( spawn_camera )                //カメラのspawn
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
{	cmds
	.spawn( Camera2dBundle::default() )
    .insert( Camera { priority: 0, ..default() } )
	;
    cmds
    .spawn( Camera3dBundle::default() )
    .insert( Camera { priority: 1, ..default() } )
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

//End of code.