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
	maze: Res<GameMap>,
	mut player_params: ResMut<PlayerParameters>,
	mut sysparams: ResMut<SystemParameters>,
	egui: Res<EguiContext>,
)
{	let tmp_darkmode = sysparams.darkmode;
	let tmp_sysinfo  = sysparams.sysinfo;

	//コンソールウィンドウを更新する
	egui::Window::new( "Control panel" ).show
	(	egui.ctx(), | ui |
		{	//挑戦中のステージは何番目か
			ui.label( format!( "Stage: {}", sysparams.stage ) );

			//次のステージの迷路作成関数を乱数で決めるか、固定するか
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

			//スキル・オートマッピングのレベル変更
			let mut tmp = *player_params.skill_set.get( SKILL_AUTO_MAPPING ).unwrap();
			ui.label( "Skill:".to_string() );
			ui.horizontal( | ui |
			{	ui.label( "- Auto Mapping:".to_string() );
				egui::ComboBox::from_id_source( "Auto Mapping Lv" )
				.width( PIXEL_PER_GRID )
				.selected_text( format!( "Lv{:?}", tmp ) )
				.show_ui( ui, | ui |
				{	ui.selectable_value( &mut tmp, 1, "Lv1" );
					ui.selectable_value( &mut tmp, 2, "Lv2" );
					ui.selectable_value( &mut tmp, 3, "Lv3" );
					ui.selectable_value( &mut tmp, 4, "Lv4" );
					ui.selectable_value( &mut tmp, 5, "Lv5" );
				} );
			} );
			if tmp != *player_params.skill_set.get( SKILL_AUTO_MAPPING ).unwrap()
			{	player_params.skill_set.insert( SKILL_AUTO_MAPPING, tmp );
			}

			//各種チェックボックス
			ui.horizontal( | ui |
			{	ui.checkbox( &mut sysparams.darkmode, "Dark mode"   ); //迷路全体非表示のOn/Off
				ui.checkbox( &mut sysparams.sysinfo , "System info" ); //迷路のシステム情報の表示On/Off
			} );
		}
	);

	//Dark modeのチェックボックスが切り替わったら
	if sysparams.darkmode != tmp_darkmode
	{	for ( id, wall ) in q_spr_wall_id.iter()
		{	if maze.is_visible( wall.x, wall.y ) { continue }
			q_visible.get_component_mut::<Visible>( id ).unwrap().is_visible = ! sysparams.darkmode;
		}
	}

	//System infoのチェックボックスが切り替わったら
	if sysparams.sysinfo != tmp_sysinfo
	{	for id in q_sysinfo_id.iter()
		{	q_visible.get_component_mut::<Visible>( id ).unwrap().is_visible = sysparams.sysinfo;
		}
	}
}

//End of code.