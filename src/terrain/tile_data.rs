use bevy::prelude::*;

use super::TILE_COUNT;
use crate::{
    app::AppUpdate,
    chunk::{Chunk, ChunkManager, LoadLevel},
};

pub struct TerrainDataPlugin;
impl Plugin for TerrainDataPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(add_terrain_data_to_chunk);
        app.add_observer(add_tile_data_to_chunk);

        app.add_event::<BrakeTile>()
            .add_systems(Update, brake_tile.in_set(AppUpdate::PostAction));
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
    Dirt,
    Sand,
}

impl TerrainType {
    pub fn get_terrain_color(&self) -> Color {
        match self {
            TerrainType::Stone => Color::srgba(0.1, 0.1, 0.2, 0.5),
            TerrainType::Dirt => Color::srgba(0.3, 0.1, 0.1, 0.5),
            TerrainType::Sand => Color::srgba(0.3, 0.8, 0.2, 0.5),
        }
    }
}

#[derive(Component, Clone, Copy, Deref, DerefMut)]
pub struct TerrainData(pub [TerrainType; TILE_COUNT]);
impl Default for TerrainData {
    fn default() -> Self {
        Self([TerrainType::default(); TILE_COUNT])
    }
}
impl TerrainData {
    pub fn get_color(&self, x: u32, y: u32) -> Color {
        let index = super::tile_index(x, y);
        self[index as usize].get_terrain_color()
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

#[derive(Event, Clone, Copy)]
//I may just want to make this an enum and have two ways to send the event. By entity or by position
/// Transfom the NonGround into a Ground tile
pub enum BrakeTile {
    ByEntity(Entity),
    ByPos(Vec3),
}

fn brake_tile(
    mut events: EventReader<BrakeTile>,
    tiles: Query<(&ChildOf, &Transform)>,
    mut tile_data: Query<&mut TileData>,
    chunk_manager: Res<ChunkManager>,
) {
    for event in events.read() {
        match event {
            BrakeTile::ByEntity(tile) => {
                let Ok((child_of, transform)) = tiles.get(*tile) else {
                    warn!("Tile does not exist: {tile}");
                    continue;
                };
                let Ok(mut tile_data) = tile_data.get_mut(child_of.parent()) else {
                    warn!("could not access tile data: {tile}");
                    continue;
                };
                let local_tile_pos = super::get_local_tile_pos(transform.translation.xy());
                let local_tile_id = super::tile_index(local_tile_pos.x, local_tile_pos.y) as usize;
                tile_data[local_tile_id] = TileType::Ground;
            }
            BrakeTile::ByPos(pos) => {
                let Some(chunk) = chunk_manager.get(Chunk::get_chunk_pos(*pos)) else {
                    warn!("No chunk at pos: {pos}");
                    continue;
                };
                let local_tile_pos = super::get_local_tile_pos(pos.xy());
                let local_tile_id = super::tile_index(local_tile_pos.x, local_tile_pos.y) as usize;
                if let Ok(mut tile_data) = tile_data.get_mut(chunk) {
                    tile_data[local_tile_id] = TileType::Ground;
                } else {
                    warn!(
                        "could not access tile data at: {chunk}. local tile pos:{local_tile_pos}"
                    );
                }
            }
        }
    }
}
