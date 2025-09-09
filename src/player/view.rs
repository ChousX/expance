use bevy::prelude::*;

use crate::{
    camera::MainCamera,
    helper::move_entity_to::{MoveEntityTo, Speed},
    player::{OwnedBy, Player},
};

pub struct PlayerViewPlugin;
impl Plugin for PlayerViewPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<MovePlayerView>()
            .add_observer(spawn_default_player_view)
            .add_observer(move_active_player_view)
            .add_observer(move_player_view_to)
            .add_observer(move_player_view_by);
    }
}

fn spawn_default_player_view(
    trigger: Trigger<OnAdd, Player>,
    camera: Query<(Entity, &GlobalTransform), With<MainCamera>>,
    mut commands: Commands,
) {
    let Ok((camera, camera_transform)) = camera.single() else {
        return;
    };
    commands
        //Building Default PlayerView
        .spawn((
            PlayerView,
            camera_transform.compute_transform(),
            OwnedBy(trigger.target()),
            Speed(5000.0),
        ))
        //Adding the camera too sead view
        .add_child(camera);
    //insuring camera translation is 0,0,0
    commands.entity(camera).insert(Transform::default());
}

#[derive(Component, Default)]
#[require(Transform)]
pub struct PlayerView;

#[derive(Event)]
pub enum MovePlayerView {
    To(Vec2),
    By(Vec2),
}

#[derive(Event)]
pub enum MoveActivePlayerView {
    To(Vec2),
    By(Vec2),
}

fn move_active_player_view(
    trigger: Trigger<MoveActivePlayerView>,
    mut commands: Commands,
    camera: Query<&ChildOf, With<MainCamera>>,
) {
    if let Ok(parent) = camera.single() {
        let move_type = match trigger.event() {
            MoveActivePlayerView::To(amount) => MovePlayerView::To(*amount),
            MoveActivePlayerView::By(amount) => MovePlayerView::By(*amount),
        };
        commands.entity(parent.parent()).trigger(move_type);
    }
}
fn move_player_view_to(
    trigger: Trigger<MovePlayerView>,
    mut commands: Commands,
    mut transforms: Query<(&GlobalTransform, Option<&mut MoveEntityTo>), With<PlayerView>>,
) {
    let &MovePlayerView::To(to) = trigger.event() else {
        return;
    };
    let target = trigger.target();
    let Ok((transform, move_entity_to)) = transforms.get_mut(target) else {
        return;
    };
    match move_entity_to {
        Some(mut move_entity_to) => {
            move_entity_to.to += to;
        }
        None => {
            let from = transform.translation().xy();
            commands.entity(target).insert(MoveEntityTo {
                to,
                from,
                easing: EaseFunction::SmootherStepIn,
            });
        }
    }
}

fn move_player_view_by(
    trigger: Trigger<MovePlayerView>,
    mut transforms: Query<&mut Transform, With<PlayerView>>,
) {
    let &MovePlayerView::By(amout) = trigger.event() else {
        return;
    };
    transforms.get_mut(trigger.target()).unwrap().translation += amout.extend(0.0);
}
