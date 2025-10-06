use bevy::prelude::*;
use bevy_asset_loader::asset_collection::AssetCollection;

use crate::{
    player::{OwnedBy, Player},
    terrain::{BrakeTile, TILE_SIZE, brake_all_tiles_around},
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
    mut tile_brakes: EventWriter<BrakeTile>,
) {
    info!("spawn player core");
    let sprite = Sprite::from_image(sprite_texture.default.clone());
    let transform = Transform::from_xyz(TILE_SIZE.x / 2.0, TILE_SIZE.y / 2.0, 1.1);
    commands.spawn((PlayerCore, transform, sprite, OwnedBy(trigger.target())));
    brake_all_tiles_around(transform.translation, 1, &mut tile_brakes);
}
