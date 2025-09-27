use bevy::prelude::*;

use super::TILE_COUNT;
use crate::chunk::LoadLevel;

pub struct TerrainDataPlugin;
impl Plugin for TerrainDataPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(add_terrain_data_to_chunk);
        app.add_observer(add_tile_data_to_chunk);
    }
}

#[derive(Clone, Copy, Default)]
pub enum TileType {
    #[default]
    Wall,
    Ground,
}

#[derive(Component, Clone, Copy)]
pub struct TileData(pub [TileType; TILE_COUNT]);

#[derive(Clone, Copy, Default)]
pub enum TerrainType {
    #[default]
    Stone,
}

#[derive(Component, Clone, Copy)]
pub struct TerrainData(pub [TerrainType; TILE_COUNT]);

fn add_terrain_data_to_chunk(trigger: Trigger<OnInsert, LoadLevel>) {}
fn add_tile_data_to_chunk(trigger: Trigger<OnInsert, LoadLevel>) {}
