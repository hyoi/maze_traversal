use super::*;

//text UIを配置する
pub fn spawn_ui_text
(   mut cmds: Commands,
    asset_svr: Res<AssetServer>,
)
{   //レイアウト用の隠しフレームを作る
    let per100 = Val::Percent( 100.0 );
    let style = Style
    {   size           : Size::new( per100, per100 ),
        position_type  : PositionType::Absolute,
        flex_direction : FlexDirection::Column,
        justify_content: JustifyContent::FlexEnd, //画面の下端
        ..default()
    };
    let background_color = BackgroundColor ( Color::NONE );
    let hidden_frame = NodeBundle { style, background_color, ..default() };

    //フッターText
    let mut ui_footer_left   = ui_text( &FOOTER_LEFT_TEXT  , TextAlignment::Center, &asset_svr );
    let mut ui_footer_center = ui_text( &FOOTER_CENTER_TEXT, TextAlignment::Center, &asset_svr );
    let mut ui_footer_right  = ui_text( &FOOTER_RIGHT_TEXT , TextAlignment::Center, &asset_svr );
    ui_footer_left.style.align_self   = AlignSelf::FlexStart;
    ui_footer_center.style.align_self = AlignSelf::Center;
    ui_footer_right.style.align_self  = AlignSelf::FlexEnd;

    //隠しフレームの中に子要素を作成する
    cmds.spawn( hidden_frame ).with_children
    (   | cmds |
        {   cmds.spawn( ( ui_footer_left  , FooterLeft   ) );
            cmds.spawn( ( ui_footer_center, FooterCenter ) );
            cmds.spawn( ( ui_footer_right , FooterRight  ) );
        }
    );
}

//text UI用にTextBundleを作る
fn ui_text
(   message: &[ MessageSect ],
    alignment: TextAlignment,
    asset_svr: &Res<AssetServer>,
) -> TextBundle
{   let mut sections = Vec::new();
    for ( line, file, size, color ) in message.iter()
    {   let value = line.to_string();
        let style = TextStyle
        {   font     : asset_svr.load( *file ),
            font_size: *size,
            color    : *color
        };
        sections.push( TextSection { value, style } );
    }
    let position_type = PositionType::Absolute;

    let text  = Text { sections, alignment, ..default() };
    let style = Style { position_type, ..default() };
    TextBundle { text, style, ..default() }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

//UIの表示を更新する(FPS)
pub fn update_fps
(   mut q: Query<&mut Text, With<FooterLeft>>,
    diag: Res<Diagnostics>,
)
{   let Ok( mut ui ) = q.get_single_mut() else { return };

    let fps_avr = diag
    .get( FrameTimeDiagnosticsPlugin::FPS )
    .map_or
    (   NA3_2.to_string(),
        | fps | fps.average().map_or
        (   NA3_2.to_string(),
            | avg | format!( "{avg:03.02}" )
        )
    );
    ui.sections[ 1 ].value = fps_avr;
}

//End of code.