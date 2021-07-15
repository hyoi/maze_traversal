use super::*;

//Pluginの手続き
pub struct PluginControlPanel;
impl Plugin for PluginControlPanel
{	fn build( &self, app: &mut AppBuilder )
	{	app
		//------------------------------------------------------------------------------------------
		.add_system( update_control_panel_window.system() )				// コンソールウィンドウの更新
		//------------------------------------------------------------------------------------------
		;
	}
}

////////////////////////////////////////////////////////////////////////////////////////////////////

//コンソールウィンドウを更新する
fn update_control_panel_window
(	mut q_visible: Query<&mut Visible>,
	q_spr_wall_id: Query<( Entity, &SpriteWall )>,
	q_sysinfo_id : Query<Entity, With<SysinfoObj>>,
	mut maze: ResMut<GameMap>,
	mut automap: ResMut<AutoMap>,
	mut sysparams: ResMut<SystemParameters>,
	egui: Res<EguiContext>,
)
{	let tmp_darkmode = maze.is_darkmode;
	let tmp_sysinfo  = maze.is_sysinfo;

	//コンソールウィンドウを更新する
	egui::Window::new( "Control panel" ).show
	(	egui.ctx(), | ui |
		{	ui.label( format!( "Stage: {}", maze.level ) );
			ui.horizontal( | ui |
			{	ui.label( "Next Maze:" );
				egui::ComboBox::from_id_source( "Next Maze" )
				.width( PIXEL_PER_GRID * 2.7 )
				.selected_text( format!( "{:?}", sysparams.maze_type ) )
				.show_ui( ui, | ui |
				{	ui.selectable_value( &mut sysparams.maze_type, SelectMazeType::Random, "Random" );
					ui.selectable_value( &mut sysparams.maze_type, SelectMazeType::Type1,  "Type1 " );
					ui.selectable_value( &mut sysparams.maze_type, SelectMazeType::Type2,  "Type2 " );
					ui.selectable_value( &mut sysparams.maze_type, SelectMazeType::Type3,  "Type3 " );
				} );
			} );
			ui.label( "Skill:".to_string() );
			ui.horizontal( | ui |
			{	ui.label( "- Auto Mapping:".to_string() );
				egui::ComboBox::from_id_source( "Auto Mapping Lv" )
				.width( PIXEL_PER_GRID )
				.selected_text( format!( "Lv{:?}", automap.0 ) )
				.show_ui( ui, | ui |
				{	ui.selectable_value( &mut automap.0, 1, "Lv1" );
					ui.selectable_value( &mut automap.0, 2, "Lv2" );
					ui.selectable_value( &mut automap.0, 3, "Lv3" );
					ui.selectable_value( &mut automap.0, 4, "Lv4" );
					ui.selectable_value( &mut automap.0, 5, "Lv5" );
				} );
			} );
			ui.horizontal( | ui |
			{	ui.checkbox( &mut maze.is_darkmode, "Dark mode"   );
				ui.checkbox( &mut maze.is_sysinfo , "System info" );
			} );
		}
	);

	//Dark modeのチェックボックスが切り替わったら
	if maze.is_darkmode != tmp_darkmode
	{	for ( id, wall ) in q_spr_wall_id.iter()
		{	if maze.is_visible( wall.x, wall.y ) { continue }
			q_visible.get_component_mut::<Visible>( id ).unwrap().is_visible = ! maze.is_darkmode;
		}
	}

	//System infoのチェックボックスが切り替わったら
	if maze.is_sysinfo != tmp_sysinfo
	{	for id in q_sysinfo_id.iter()
		{	q_visible.get_component_mut::<Visible>( id ).unwrap().is_visible = maze.is_sysinfo;
		}
	}
}

//End of code.