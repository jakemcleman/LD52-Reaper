use bevy::prelude::*;

use crate::actions::game_control::{get_movement, GameControl};
use crate::GameState;

use self::game_control::get_gamepad_movement;

mod game_control;

pub struct ActionsPlugin;

// This plugin listens for keyboard input and converts the input into Actions
// Actions can then be used as a resource in other systems to act on the player input.
impl Plugin for ActionsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Actions>().add_system_set(
            SystemSet::on_update(GameState::Playing).with_system(set_movement_actions),
        );
    }
}

#[derive(Default, Resource)]
pub struct Actions {
    pub player_movement: Vec2,
    pub jump: bool,
}

pub fn set_movement_actions(
    mut actions: ResMut<Actions>, 
    keyboard_input: Res<Input<KeyCode>>, 
    gamepad_input: Res<Gamepads>,
    gamepad_buttons: Res<Input<GamepadButton>>,
    gamepad_axes: Res<Axis<GamepadAxis>>
) {
    let gamepad_movement = get_gamepad_movement(&gamepad_input, &gamepad_buttons, &gamepad_axes);
    
    let keyboard_movement = Vec2::new(
        get_movement(GameControl::Right, &keyboard_input)
            - get_movement(GameControl::Left, &keyboard_input),
        get_movement(GameControl::Up, &keyboard_input)
            - get_movement(GameControl::Down, &keyboard_input),
    );

    if keyboard_movement != Vec2::ZERO {
        actions.player_movement = keyboard_movement.normalize();
    } else if gamepad_movement.length_squared() > 0.1 {
        actions.player_movement = gamepad_movement.normalize();
    } else {
        actions.player_movement = Vec2::ZERO;
    }
    
    actions.jump = keyboard_input.pressed(KeyCode::Space);
    
    for gamepad in gamepad_input.iter() { 
        actions.jump = actions.jump || gamepad_buttons.pressed(GamepadButton::new(gamepad, GamepadButtonType::South));
    }
}
