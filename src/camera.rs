use bevy::prelude::*;

pub struct CameraPlugin;
impl Plugin for CameraPlugin{
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera);
    }
}

#[derive(Component, Default)]
#[require(Camera2d)]
pub struct MainCamera;

fn spawn_camera(mut commands: Commands){
    commands.spawn(MainCamera);
}
