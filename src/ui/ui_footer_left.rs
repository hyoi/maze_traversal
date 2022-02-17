use super::*;

//external modules
use bevy::diagnostic::*;

//Pluginの手続き
pub struct PluginUiFooterLeft;
impl Plugin for PluginUiFooterLeft
{	fn build( &self, app: &mut App )
	{	app
		//------------------------------------------------------------------------------------------
		.add_plugin( FrameTimeDiagnosticsPlugin::default() )	// fps計測のプラグイン
		//------------------------------------------------------------------------------------------
		.add_system( update_ui_footer_left  )					// UIの表示を更新
		//------------------------------------------------------------------------------------------
		;
	}
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Component)]
pub struct UiFooterLeft;
pub const UI_FOOTER_LEFT: [ MessageSect; 2 ] =
[	( " FPS ", FONT_ORBITRON_BLACK, PIXEL_PER_GRID * 0.8, Color::ORANGE ),
	( ""     , FONT_ORBITRON_BLACK, PIXEL_PER_GRID * 1.0, Color::WHITE  ),
];

////////////////////////////////////////////////////////////////////////////////////////////////////

//下端の情報表示を更新する(左)
fn update_ui_footer_left
(	mut q: Query< &mut Text, With<UiFooterLeft>>,
	diag: Res<Diagnostics>,
)
{	if let Ok( mut ui ) = q.get_single_mut()
	{	let fps_avr = diag.get( FrameTimeDiagnosticsPlugin::FPS ).map_or
		(	NA_STR4.to_string(),
			| fps | match fps.average()
			{	Some( avg ) => format!( "{:2.2}", avg ),
				None        => NA_STR4.to_string()
			}
		);
		ui.sections[ 1 ].value = fps_avr;
	}
}

//End of code.