use bevy::prelude::*;

use crate::player::{OwnedBy, Player};

pub struct PlayerCorePlugin;
impl Plugin for PlayerCorePlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(spawn_player_core);
    }
}

#[derive(Component, Default)]
pub struct PlayerCore;

fn spawn_player_core(
    trigger: Trigger<OnAdd, Player>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    info!("spawn player core");
    let sprite = Sprite::from_image(asset_server.load("placeholder/Diamond/Sprite-0001.png"));
    //  find good spot to place
    let transform = Transform::default();
    commands.spawn((PlayerCore, transform, sprite, OwnedBy(trigger.target())));
}
