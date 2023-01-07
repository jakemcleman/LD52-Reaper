use bevy::prelude::*;

pub enum GameControl {
    Up,
    Down,
    Left,
    Right,
}

impl GameControl {
    pub fn pressed(&self, keyboard_input: &Res<Input<KeyCode>>) -> bool {
        match self {
            GameControl::Up => {
                keyboard_input.pressed(KeyCode::W) || keyboard_input.pressed(KeyCode::Up)
            }
            GameControl::Down => {
                keyboard_input.pressed(KeyCode::S) || keyboard_input.pressed(KeyCode::Down)
            }
            GameControl::Left => {
                keyboard_input.pressed(KeyCode::A) || keyboard_input.pressed(KeyCode::Left)
            }
            GameControl::Right => {
                keyboard_input.pressed(KeyCode::D) || keyboard_input.pressed(KeyCode::Right)
            }
        }
    }
}

pub fn get_movement(control: GameControl, keyinput: &Res<Input<KeyCode>>) -> f32 {
    if control.pressed(keyinput) {
        1.0
    } else {
        0.0
    }
}

pub fn get_gamepad_movement (
    gamepads: &Res<Gamepads>,
    button_inputs: &Res<Input<GamepadButton>>,
    axes: &Res<Axis<GamepadAxis>>,
) -> Vec2 {
    let mut movement = Vec2::ZERO;
    for gamepad in gamepads.iter() {        
        movement.x += axes.get(GamepadAxis::new(gamepad, GamepadAxisType::LeftStickX)).unwrap_or(0.);
        movement.y += axes.get(GamepadAxis::new(gamepad, GamepadAxisType::LeftStickY)).unwrap_or(0.);
    
        if button_inputs.pressed(GamepadButton::new(gamepad, GamepadButtonType::DPadLeft)) {
            movement.x -= 1.;
        }
        if button_inputs.pressed(GamepadButton::new(gamepad, GamepadButtonType::DPadRight)) {
            movement.x += 1.;
        }
        if button_inputs.pressed(GamepadButton::new(gamepad, GamepadButtonType::DPadDown)) {
            movement.y -= 1.;
        }
        if button_inputs.pressed(GamepadButton::new(gamepad, GamepadButtonType::DPadUp)) {
            movement.y += 1.;
        }
    }
    movement
}