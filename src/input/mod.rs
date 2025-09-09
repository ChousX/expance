use bevy::prelude::*;
mod keyboard;

pub struct InputPlugin;
impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(keyboard::KeyboardPlugin);
    }
}
