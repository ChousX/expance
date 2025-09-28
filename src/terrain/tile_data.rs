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

#[derive(Component, Clone, Copy, Deref, DerefMut)]
pub struct TileData(pub [TileType; TILE_COUNT]);

impl TileData {
    pub fn get_texture_index(&self, x: u32, y: u32) -> u32 {
        let index = super::tile_index(x, y);
        match self[index as usize] {
            TileType::Wall => 1,
            TileType::Ground => 2,
        }
    }
}

impl Default for TileData {
    fn default() -> Self {
        Self([TileType::default(); TILE_COUNT])
    }
}

#[derive(Clone, Copy, Default)]
pub enum TerrainType {
    #[default]
    Stone,
}

#[derive(Component, Clone, Copy)]
pub struct TerrainData(pub [TerrainType; TILE_COUNT]);
impl Default for TerrainData {
    fn default() -> Self {
        Self([TerrainType::default(); TILE_COUNT])
    }
}

fn add_terrain_data_to_chunk(
    trigger: Trigger<OnInsert, LoadLevel>,
    chunks: Query<&LoadLevel>,
    mut commands: Commands,
) {
    let Ok(load_level) = chunks.get(trigger.target()) else {
        return;
    };
    if load_level < &LoadLevel::Mostly {
        return;
    }
    commands
        .entity(trigger.target())
        .insert(TerrainData::default());
}

fn add_tile_data_to_chunk(
    trigger: Trigger<OnInsert, LoadLevel>,
    chunks: Query<&LoadLevel>,
    mut commands: Commands,
) {
    let Ok(load_level) = chunks.get(trigger.target()) else {
        return;
    };
    if load_level < &LoadLevel::Mostly {
        return;
    }
    commands
        .entity(trigger.target())
        .insert(TileData::default());
}
