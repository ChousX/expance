use crate::{app::AppUpdate, player::view::MoveActivePlayerView};
use bevy::prelude::*;

pub struct KeyboardPlugin;
impl Plugin for KeyboardPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<KeyboardBindings>()
            .add_systems(Update, move_current_view.in_set(AppUpdate::PreData));
    }
}

#[derive(Resource)]
pub struct KeyboardBindings {
    pub move_up: [Option<KeyCode>; 2],
    pub move_down: [Option<KeyCode>; 2],
    pub move_left: [Option<KeyCode>; 2],
    pub move_right: [Option<KeyCode>; 2],
}

impl KeyboardBindings {
    pub fn is_pressed_up(&self, keys: &Res<ButtonInput<KeyCode>>) -> bool {
        keys.any_pressed(self.move_up.iter().filter_map(|&v| v))
    }
    pub fn is_pressed_down(&self, keys: &Res<ButtonInput<KeyCode>>) -> bool {
        keys.any_pressed(self.move_down.iter().filter_map(|&v| v))
    }
    pub fn is_pressed_left(&self, keys: &Res<ButtonInput<KeyCode>>) -> bool {
        keys.any_pressed(self.move_left.iter().filter_map(|&v| v))
    }
    pub fn is_pressed_right(&self, keys: &Res<ButtonInput<KeyCode>>) -> bool {
        keys.any_pressed(self.move_right.iter().filter_map(|&v| v))
    }
}

impl Default for KeyboardBindings {
    fn default() -> Self {
        Self {
            move_up: [Some(KeyCode::KeyW), Some(KeyCode::KeyK)],
            move_down: [Some(KeyCode::KeyS), Some(KeyCode::KeyJ)],
            move_left: [Some(KeyCode::KeyA), Some(KeyCode::KeyH)],
            move_right: [Some(KeyCode::KeyD), Some(KeyCode::KeyL)],
        }
    }
}

fn move_current_view(
    keyboard: Res<ButtonInput<KeyCode>>,
    bindings: Res<KeyboardBindings>,
    mut commands: Commands,
) {
    let mut amount = Vec2::ZERO;
    if bindings.is_pressed_up(&keyboard) {
        amount.y += 1.0;
    }
    if bindings.is_pressed_down(&keyboard) {
        amount.y -= 1.0;
    }
    if bindings.is_pressed_left(&keyboard) {
        amount.x -= 1.0;
    }
    if bindings.is_pressed_right(&keyboard) {
        amount.x += 1.0;
    }
    commands.trigger(MoveActivePlayerView::By(amount * 5.0));
}
