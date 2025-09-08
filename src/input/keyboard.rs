use bevy::prelude::*;

pub struct KeyboardPlugin;
impl Plugin for KeyboardPlugin {
    fn build(&self, app: &mut App) {}
}

pub struct KeyboardBindings {
    move_up: [Option<KeyCode>; 2],
    move_down: [Option<KeyCode>; 2],
    move_left: [Option<KeyCode>; 2],
    move_right: [Option<KeyCode>; 2],
}

impl Default for KeyboardBindings {
    fn default() -> Self {
        Self {
            move_up: [Some(KeyCode::KeyW), None],
            move_down: [Some(KeyCode::KeyS), None],
            move_left: [Some(KeyCode::KeyA), None],
            move_right: [Some(KeyCode::KeyD), None],
        }
    }
}
