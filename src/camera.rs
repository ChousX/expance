use bevy::prelude::*;

use crate::{
    app::AppState,
    chunk::{Chunk, ChunkLoader},
};

pub struct CameraPlugin;
impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera)
            .add_systems(OnEnter(AppState::Game), add_chunk_loader_to_camera);
    }
}

#[derive(Component, Default)]
#[require(Camera2d)]
pub struct MainCamera;

fn spawn_camera(mut commands: Commands) {
    commands.spawn((MainCamera,));
}

fn add_chunk_loader_to_camera(mut camera: Query<Entity, With<MainCamera>>, mut commands: Commands) {
    let camera = camera
        .single_mut()
        .expect("Getting only one MainCamera faild");
    commands.entity(camera).insert(ChunkLoader {
        full: Chunk::SIZE,
        mostly: Chunk::SIZE * 2.0,
        minimum: Chunk::SIZE * 4.0,
    });
}
