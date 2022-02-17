use super::*;

//Pluginの手続き
pub struct PluginUiHeaderRight;
impl Plugin for PluginUiHeaderRight
{	fn build( &self, app: &mut App )
	{	app
		//------------------------------------------------------------------------------------------
		.add_system( update_ui_header_right )			// UIの表示を更新
		//------------------------------------------------------------------------------------------
		;
	}
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Component)]
pub struct UiHeaderRight;
pub const UI_HEADER_RIGHT: [ MessageSect; 4 ] =
[	( ""        , FONT_ORBITRON_BLACK, PIXEL_PER_GRID * 1.0, Color::WHITE  ),
	( " GOLD / ", FONT_ORBITRON_BLACK, PIXEL_PER_GRID * 0.8, Color::ORANGE ),
	( ""        , FONT_ORBITRON_BLACK, PIXEL_PER_GRID * 1.0, Color::WHITE  ),
	( " FLOOR " , FONT_ORBITRON_BLACK, PIXEL_PER_GRID * 0.8, Color::ORANGE ),
];

////////////////////////////////////////////////////////////////////////////////////////////////////

//スコアとステージの表示を更新する
fn update_ui_header_right
(	mut q: Query< &mut Text, With<UiHeaderRight>>,
	o_record: Option<Res<Record>>,
)
{	if let Ok( mut ui ) = q.get_single_mut()
	{	let ( score, stage ) = o_record.map_or
		(	( NA_STR5.to_string(), NA_STR5.to_string() ),
			| x | ( x.score.to_string(), x.stage.to_string() )
		);
		ui.sections[ 0 ].value = score;
		ui.sections[ 2 ].value = stage;
	}
}

//End of code.