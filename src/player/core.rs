use bevy::prelude::*;
use bevy_asset_loader::asset_collection::AssetCollection;

use crate::{
    player::{OwnedBy, Player},
    terrain::{BrakeTile, NeedsTileBreaking, TILE_SIZE},
};

pub struct PlayerCorePlugin;
impl Plugin for PlayerCorePlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(spawn_player_core);
    }
}

#[derive(Component, Default)]
pub struct PlayerCore;

#[derive(AssetCollection, Resource)]
pub struct PlayerCoreSprite {
    #[asset(path = "placeholder/Diamond/Sprite-0001.png")]
    pub default: Handle<Image>,
}

fn spawn_player_core(
    trigger: Trigger<OnAdd, Player>,
    mut commands: Commands,
    sprite_texture: Res<PlayerCoreSprite>,
) {
    info!("spawn player core");
    let sprite = Sprite::from_image(sprite_texture.default.clone());
    let transform = Transform::from_xyz(0.0, 0.0, 1.0);
    commands.spawn((
        PlayerCore,
        transform,
        sprite,
        OwnedBy(trigger.target()), NeedsTileBreaking,
    ));
}
