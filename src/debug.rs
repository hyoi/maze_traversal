use super::*;

//external modules
//use bevy_inspector_egui::WorldInspectorPlugin;
use smooth_bevy_cameras::
{   controllers::orbit::
    {   OrbitCameraBundle,
        OrbitCameraController,
        OrbitCameraPlugin
    },
    LookTransformPlugin,
};

////////////////////////////////////////////////////////////////////////////////////////////////////

#[ derive( Resource ) ]
pub struct MarkDebugPlugin; //ロード済みフラグ

pub struct MyDebug;
impl Plugin for MyDebug
{   fn build( &self, app: &mut App )
    {   app
        .insert_resource( MarkDebugPlugin )         //ロード済みフラグ
//      .add_plugin( WorldInspectorPlugin::new() )  //Worldのパラメータ表示
        .add_plugin( LookTransformPlugin )          //オービットカメラ(1)
        .add_plugin( OrbitCameraPlugin::default() ) //オービットカメラ(2)
        .add_startup_system( spawn_orbit_camera )   //オービットカメラ(3)
        ;
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

//オービットカメラを作る
fn spawn_orbit_camera
(   mut cmds: Commands,
)
{   cmds
    .spawn( Camera3dBundle::default() )
    .insert( Camera { priority: 1, ..default() } )
    .insert
    (   OrbitCameraBundle::new
        (   OrbitCameraController::default(),
            Vec3::new( -10.0, 30.0, 20.0 ),
            Vec3::new( 0.0, 0.0, 0.0 ),
        )
    );
}

//End of code.