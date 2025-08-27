use crate::app::AppUpdate;
use crate::camera::MainCamera;
use crate::chunk::{Chunk, ChunkLoader};
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

pub struct CurrsorPlugin;
impl Plugin for CurrsorPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CurrsorPositon>()
            .add_systems(
                Update,
                (update_currsor_pos, update_transform_cursor_entity)
                    .chain()
                    .in_set(AppUpdate::PreData),
            )
            .add_systems(Startup, spawn_cursor_entity);
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

#[derive(Component, Default)]
#[require(Transform)]
pub struct CursorEntity;

fn spawn_cursor_entity(mut commands: Commands) {
    commands.spawn((
        CursorEntity,
        ChunkLoader {
            full: vec2(100.0, 100.0),
            mostly: vec2(500.0, 500.0),
            minimum: vec2(1000.0, 1000.0),
        },
    ));
}

fn update_transform_cursor_entity(
    pos: Res<CurrsorPositon>,
    mut transform: Query<&mut Transform, With<CursorEntity>>,
) {
    if !pos.is_changed() {
        return;
    }
    let Ok(mut transform) = transform.single_mut() else {
        return;
    };
    transform.translation = pos.extend(transform.translation.z);
}
