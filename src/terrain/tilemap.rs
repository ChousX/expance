use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

use crate::chunk::LoadLevel;

pub struct TerrainTilemapPlugin;
impl Plugin for TerrainTilemapPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(add_tilemap_to_chunk);
        app.add_systems(Update, update_tilemap);
    }
}

fn add_tilemap_to_chunk(trigger: Trigger<OnInsert, LoadLevel>) {
    //Check if the load level is >= to LoadLevel::Full

    //Check if the Tilemap already exists

    //Check if requisit data is available

    //Init Tilemap
}

fn update_tilemap(chunks: Query<(&TileStorage), Changed<LoadLevel>>) {
    //Update Tiles to reflect terrain data
}
