//import external modules
use bevy::{ prelude::*, diagnostic::*, input::gamepad::* };
use rand::prelude::*;
use counted_array::*;
use once_cell::sync::*;

//local crates
use macros::*;
use official_examples::*;

//internal submodules
mod a_public;
mod b_init_app;
// mod map;

use a_public::*;
use b_init_app::InitApp;
// use map::PluginMap;

// mod player;
// mod chasers;

// use player::PluginPlayer;
// use chasers::PluginChaser;

//メイン関数
fn main()
{   //ログのコンソールへの出力を抑止
    #[cfg( not( target_arch = "wasm32" ) )]
    std::env::set_var( "RUST_LOG", "OFF" );

    //アプリの実行
    let mut app = App::new();
    app
    .add_plugin( InitApp )
    // .add_plugin( PluginMap )
    // .add_plugin( PluginPlayer )
    // .add_plugin( PluginChaser )
    ;

    if DEBUG() { app.add_plugin( gamepad_viewer::GamepadViewer ); }
    app.run();
}

//End of code.