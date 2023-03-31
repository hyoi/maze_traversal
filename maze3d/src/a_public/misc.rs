use super::*;

//submodules
mod now_loading;
pub use now_loading::*; //re-export

//ウィンドウとフルスクリーンの切換(WASMでは不要)
pub fn toggle_window_mode
(   mut q: Query<&mut Window>,
    inkey: Res<Input<KeyCode>>,
    inbtn: Res<Input<GamepadButton>>,
    mut button_events: EventReader<GamepadButtonChangedEvent>,
)
{   //ウィンドウが見つからないなら
    let Ok( mut window ) = q.get_single_mut() else { return };

    //Alt＋Enterキーの状態
    let is_key_pressed =
        ( inkey.pressed( KeyCode::RAlt ) || inkey.pressed( KeyCode::LAlt ) )
            && inkey.just_pressed( KeyCode::Return );

    //パッドのボタンの状態
    let mut is_btn_pressed = false;
    let pause_btn = GamepadButtonType::Select;
    for button_event in button_events.iter()
    {   if button_event.button_type == pause_btn
        {   let btn = GamepadButton::new( button_event.gamepad, pause_btn );
            is_btn_pressed = inbtn.just_pressed( btn );
            if is_btn_pressed { break }
        }
    }

    //入力がないなら
    if ! is_key_pressed && ! is_btn_pressed { return }

    //ウィンドウとフルスクリーンを切り替える
    use bevy::window::WindowMode::*;
    window.mode = match window.mode
    {   Windowed => SizedFullscreen,
        _        => Windowed,
    };
}

//QueryしたEnityを再帰的に削除する
pub fn despawn_entity<T: Component>
(   q: Query<Entity, With<T>>,
    mut cmds: Commands,
)
{   q.for_each( | id | cmds.entity( id ).despawn_recursive() );
}

// UI Textを表示する
// pub fn show_ui<T: Component>( mut q: Query<&mut Visibility, With<T>> )
// {	let _ = q.get_single_mut().map( | mut ui | ui.is_visible = true );
// }

// UI Textを隠す
// pub fn hide_ui<T: Component>( mut q: Query<&mut Visibility, With<T>> )
// {	let _ = q.get_single_mut().map( | mut ui | ui.is_visible = false );
// }

//End of code.