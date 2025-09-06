use bevy::prelude::*;

pub struct HelperPlugin;
impl Plugin for HelperPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(move_entity_to::MoveEntityToPlugin);
    }
}

pub mod move_entity_to;

mod create_texture_atlas;
pub use create_texture_atlas::create_texture_atlas;
