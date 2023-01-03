use super::*;

//submodules
// mod fetch_assets;
// mod spawn_text_ui;

// use fetch_assets::*;
// use spawn_text_ui::*;

//プラグインの設定
pub struct InitApp;
impl Plugin for InitApp
{   fn build( &self, app: &mut App )
    {   //メインウィンドウ、背景色、アンチエイリアシング、プラグイン
        let window = WindowDescriptor
        {   title    : APP_TITLE.to_string(),
            width    : WINDOW_PIXELS_WIDTH,
            height   : WINDOW_PIXELS_HEIGHT,
            resizable: false,
            // fit_canvas_to_parent: true, //FIX v0.6.1: Android Chromeで発生する不具合を回避
            ..default()
        };
        app
        .insert_resource( ClearColor( WINDOW_BACKGROUND_COLOR ) )
        .insert_resource( Msaa { samples: 4 } )
        .add_plugins( DefaultPlugins.set( WindowPlugin { window, ..default() } ) )
        ;
    }
}

//End of code.