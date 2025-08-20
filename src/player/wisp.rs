use crate::app::AppUpdate;
use crate::cursor::CurrsorPositon;
use crate::helper::move_entity_to::{MoveEntityTo, Speed};
use crate::player::OwnedBy;
use crate::player::core::PlayerCore;
use bevy::prelude::*;

pub struct PlayerWispPlugin;
impl Plugin for PlayerWispPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(spawn_player_wisp);
        app.add_systems(
            Update,
            (update_move_to, add_move_to)
                .chain()
                .in_set(AppUpdate::Data),
        );
    }
}

#[derive(Component, Default)]
#[require(Transform)]
pub struct PlayerWisp;

fn spawn_player_wisp(
    trigger: Trigger<OnAdd, PlayerCore>,
    mut commands: Commands,
    transforms: Query<(&Transform, &OwnedBy)>,
    asset_server: Res<AssetServer>,
) {
    let sprite = Sprite::from_image(asset_server.load("placeholder/Diamond/Sprite-0002.png"));
    let (&transform, &OwnedBy(owner)) = transforms.get(trigger.target()).unwrap();
    commands.spawn((PlayerWisp, sprite, transform, Speed(5000.0), OwnedBy(owner)));
    info!("spawned player wisp");
}

fn add_move_to(
    mut commands: Commands,
    cursor_pos: Res<CurrsorPositon>,
    wisp_q: Query<(&GlobalTransform, Entity), With<PlayerWisp>>,
) {
    for (wisp_transform, entity) in wisp_q.iter() {
        let wisp_pos = wisp_transform.translation().xy();
        if wisp_pos != **cursor_pos {
            commands.entity(entity).insert(MoveEntityTo {
                to: **cursor_pos,
                from: wisp_pos,
                easing: EaseFunction::SmootherStepIn,
            });
        }
    }
}

fn update_move_to(
    mut wisp_q: Query<&mut MoveEntityTo, With<PlayerWisp>>,
    cursor_pos: Res<CurrsorPositon>,
) {
    for mut move_to in wisp_q.iter_mut() {
        if move_to.to != **cursor_pos {
            move_to.to = **cursor_pos;
        }
    }
}
