use super::*;

//Pluginの手続き
pub struct PluginGamePlay;
impl Plugin for PluginGamePlay
{	fn build( &self, app: &mut AppBuilder )
	{	app
		//------------------------------------------------------------------------------------------
		.add_system_set											//GameState::GameStart
		(	SystemSet::on_enter( GameState::GameStart )			// on_enter()
				.label( MarkerLabel )
				.with_system( spawn_sprite_new_map.system() )	// 新マップを生成して表示
		)
		.add_system_set											//GameState::GameStart
		(	SystemSet::on_enter( GameState::GameStart )			// on_enter()
				.after( MarkerLabel )
				.with_system( spawn_sprite_player.system() )	// マップ生成後に自機を配置
		)
		.add_system_set											//GameState::GameStart
		(	SystemSet::on_update( GameState::GameStart )		// on_update()
				.with_system( goto_gameplay_state.system() )	// 無条件にGamePlayへ遷移
		)
		//------------------------------------------------------------------------------------------
		.add_system_set											//GameState::GamePlay
		(	SystemSet::on_update( GameState::GamePlay )			// on_update()
				.label( MarkerLabel )
				.with_system( move_sprite_player.system() )		// 自機の移動
				.with_system( animate_goal_sprite.system() )	// ゴールスプライトのアニメーション
		)
		.add_system_set											//GameState::GamePlay
		(	SystemSet::on_update( GameState::GamePlay )			// on_update()
				.after( MarkerLabel )
				.with_system( goto_state_event_queue.system() )	// eventにセットされたsutateへ遷移
		)
		//------------------------------------------------------------------------------------------
		.add_system_set											//GameState::GameClear
		(	SystemSet::on_enter( GameState::GameClear )			// on_enter()
				.with_system( show_message_clear.system() )		// クリアメッセージを表示する
		)
		.add_system_set											//GameState::GameClear
		(	SystemSet::on_update( GameState::GameClear )		// on_update()
				.with_system( countdown_gameclear.system() )	// カウントダウン後にGameStartへ遷移
		)
		.add_system_set											//GameState::GameClear
		(	SystemSet::on_exit( GameState::GameClear )			// on_exit()
				.with_system( hide_message_clear.system() )		// クリアメッセージを隠す
				.with_system( despawn_sprite_map.system() )		// マップを削除
				.with_system( despawn_sprite_player.system() )	// 自機を削除
		)
		//------------------------------------------------------------------------------------------
		;
	}
}

////////////////////////////////////////////////////////////////////////////////////////////////////

//定義と定数

#[derive(Clone,Debug,Eq,PartialEq,Hash,SystemLabel)]
struct MarkerLabel;

//CountdownTimer
#[derive(Default)]
struct CountDown { timer: Timer }

////////////////////////////////////////////////////////////////////////////////////////////////////

//無条件にGameState::GamePlayへ遷移する
fn goto_gameplay_state( mut state: ResMut<State<GameState>> )
{	state.overwrite_set( GameState::GamePlay ).unwrap();
}

//eventで渡されたstateへ遷移する(キューの先頭だけ処理)
pub fn goto_state_event_queue( mut state : ResMut<State<GameState>>, mut events: EventReader<GameState> )
{	if let Some( next_state ) = events.iter().next()
	{	state.overwrite_set( *next_state ).unwrap();
	}
}

//二次元配列の添え字から画面座標を算出する
pub fn conv_sprite_coordinates( x: usize, y: usize ) -> ( f32, f32 )
{	let x = ( PIXEL_PER_GRID - SCREEN_WIDTH  ) / 2. + PIXEL_PER_GRID * x as f32;
	let y = ( SCREEN_HEIGHT - PIXEL_PER_GRID ) / 2. - PIXEL_PER_GRID * y as f32;
	( x, y )
}

////////////////////////////////////////////////////////////////////////////////////////////////////

//クリアメッセージを表示
fn show_message_clear( mut q_ui: Query<&mut Visible, With<MessageClear>> )
{	if let Ok( mut ui ) = q_ui.single_mut() { ui.is_visible = true }
}

//ゲームクリアのカウントダウン
fn countdown_gameclear
(	mut q_ui: Query<(&mut Text, &mut Visible), With<MessageClear>>,
	mut state: ResMut<State<GameState>>,
	( mut count, mut countdown ): ( Local<i32>, Local<CountDown> ),
	time: Res<Time>,
)
{	if let Ok( ( mut text, mut visible ) ) = q_ui.single_mut()
	{	if *count <= 0
		{	//カウントダウン開始
			countdown.timer = Timer::from_seconds( 1.0, false ); //1秒タイマー
			*count = 4;
			text.sections[ 2 ].value = format!( "{}", *count - 1 );
			visible.is_visible = true;
		}
		else if countdown.timer.tick( time.delta() ).finished() //1秒経過
		{	*count -= 1;
			countdown.timer.reset();	//1秒タイマーリセット

			if *count > 0
			{	//メッセージの書き換え
				text.sections[ 2 ].value = format!( "{}", *count - 1 );
			}
			else
			{	//カウントダウンが終わったら、GameStartへ遷移する
				visible.is_visible = false;
				state.overwrite_set( GameState::GameStart ).unwrap();
			}
		}
	}
}

//クリアメッセージを隠す
fn hide_message_clear( mut q_ui: Query<&mut Visible, With<MessageClear>> )
{	if let Ok( mut ui ) = q_ui.single_mut() { ui.is_visible = false }
}

//End of code.