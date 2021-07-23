use super::*;

//Pluginの手続き
pub struct PluginEvent;
impl Plugin for PluginEvent
{	fn build( &self, app: &mut AppBuilder )
	{	app
		//------------------------------------------------------------------------------------------
		.add_system_set													// ＜GameState::Event＞
		(	SystemSet::on_enter( GameState::Event )						// ＜on_enter()＞
				.with_system( show_event_message.system() )				// EVENTメッセージを表示する
		)
		.add_system_set													// ＜GameState::Event＞
		(	SystemSet::on_update( GameState::Event )					// ＜on_update()＞
				.with_system( handle_space_key_for_event.system() )		// [Space]入力⇒GameState::Playへ遷移
		)
		// .add_system_set												// ＜GameState::Event＞
		// (	SystemSet::on_exit( GameState::Event )					// ＜on_exit()＞
		// 		.with_system( hide_event_message.system() )				// EVENTメッセージを隠す
		// )
		//------------------------------------------------------------------------------------------
		;
	}
}

////////////////////////////////////////////////////////////////////////////////////////////////////

//定義と定数

////////////////////////////////////////////////////////////////////////////////////////////////////

//EVENTメッセージ表示
fn show_event_message( mut q: Query<&mut Visible, With<MessageEvent>> )
{	if let Ok( mut ui ) = q.single_mut() { ui.is_visible = true }
}

// //EVENTメッセージ非表示
// fn hide_event_message( mut q: Query<&mut Visible, With<MessageEvent>> )
// {	if let Ok( mut ui ) = q.single_mut() { ui.is_visible = false }
// }

//[Space]が入力さたら、Playへ遷移
fn handle_space_key_for_event
(	mut q: Query<&mut Visible, With<MessageEvent>>,
	mut state: ResMut<State<GameState>>,
	mut inkey: ResMut<Input<KeyCode>>,
)
{	if ! inkey.just_pressed( KeyCode::Space ) { return }

	if let Ok( mut ui ) = q.single_mut() { ui.is_visible = false }

	let _ = state.overwrite_set( GameState::Play );
	inkey.reset( KeyCode::Escape ); // https://bevy-cheatbook.github.io/programming/states.html#with-input
}

//End of code.