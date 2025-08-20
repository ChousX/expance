use crate::app::AppUpdate;
use crate::camera::MainCamera;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

pub struct CurrsorPlugin;
impl Plugin for CurrsorPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CurrsorPositon>()
            .add_systems(Update, update_currsor_pos.in_set(AppUpdate::PreData));
    }
}

#[derive(Default, Resource, Deref, DerefMut)]
pub struct CurrsorPositon(pub Vec2);

fn update_currsor_pos(
    mut currsor_positon: ResMut<CurrsorPositon>,
    window: Query<&Window, With<PrimaryWindow>>,
    camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
    let (camera, camera_transform) = camera.single().unwrap();

    let window = window.single().unwrap();

    if let Some(world_position) = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor).ok())
        .map(|ray| ray.origin.truncate())
    {
        **currsor_positon = world_position;
    }
}
