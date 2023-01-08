//external modules
use bevy::prelude::*;
use smooth_bevy_cameras::
{   controllers::orbit::
    {   OrbitCameraBundle,
        OrbitCameraController,
        OrbitCameraPlugin
    },
    LookTransformPlugin,
};
use rand::prelude::*;

//internal modules
mod public;
use public::*;

mod init_app;
use init_app::*;

mod map;
// mod player;
// mod chasers;

use map::PluginMap;
// use player::PluginPlayer;
// use chasers::PluginChaser;

//メイン関数
fn main()
{   App::new()
    .add_plugin( InitApp )
    .add_plugin( PluginMap )
    // .add_plugin( PluginPlayer )
    // .add_plugin( PluginChaser )
    .run(); // アプリの実行
}

//End of code.