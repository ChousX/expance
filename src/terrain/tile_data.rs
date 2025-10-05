use bevy::prelude::*;

use super::{TILE_COUNT, TILE_SIZE, TILES_PRE_CHUNK};
use crate::chunk::{Chunk, ChunkManager};

pub struct TerrainDataPlugin;
impl Plugin for TerrainDataPlugin {
    fn build(&self, app: &mut App) {}
}

#[derive(Clone, Copy, Default, Component)]
pub enum TileType {
    #[default]
    Wall,
    Ground,
}
impl TileType {
    pub fn get_texture_index(&self) -> u32 {
        match self {
            TileType::Wall => 1,
            TileType::Ground => 2,
        }
    }
    pub fn generate(x: u32, y: u32, chunk_pos: IVec3) -> Self {
        Self::Wall
    }
}

#[derive(Clone, Copy, Default, Component)]
pub enum TerrainType {
    #[default]
    Stone,
    Dirt,
    Sand,
}

impl TerrainType {
    pub fn get_color(&self) -> Color {
        match self {
            TerrainType::Stone => Color::srgba(0.1, 0.1, 0.2, 0.5),
            TerrainType::Dirt => Color::srgba(0.3, 0.1, 0.1, 0.5),
            TerrainType::Sand => Color::srgba(0.3, 0.8, 0.2, 0.5),
        }
    }

    pub fn generate(x: u32, y: u32, chunk_pos: IVec3) -> Self {
        TerrainType::Stone
    }
}

#[derive(Event, Clone, Copy)]
pub enum BrakeTile {
    ByEntity(Entity),
    ByPos(Vec3),
}

fn brake_tile(
    mut events: EventReader<BrakeTile>,
    tiles: Query<(&ChildOf, &Transform)>,
    chunk_manager: Res<ChunkManager>,
) {
    for event in events.read() {
        match event {
            BrakeTile::ByEntity(tile) => {}
            BrakeTile::ByPos(pos) => {}
        }
    }
}

/// Marker component that need their surrounding tiles broken
#[derive(Component)]
pub struct NeedsTileBreaking;
