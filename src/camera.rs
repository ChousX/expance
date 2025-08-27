use bevy::prelude::*;

use crate::chunk::ChunkLoader;

pub struct CameraPlugin;
impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera);
    }
}

#[derive(Component, Default)]
#[require(Camera2d)]
pub struct MainCamera;

fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        MainCamera,
        ChunkLoader {
            full: vec2(100.0, 100.0),
            mostly: vec2(500.0, 500.0),
            minimum: vec2(1000.0, 1000.0),
        },
    ));
}
