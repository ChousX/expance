use std::ops::{Add, Mul};

use bevy::prelude::*;
use bevy_asset_loader::asset_collection::AssetCollection;

use crate::chunk::Chunk;

mod tile_data;
mod tilemap;

pub use tile_data::{BrakeTile, brake_all_tiles_around};

pub struct TerrainPlugin;
impl Plugin for TerrainPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((tile_data::TerrainDataPlugin, tilemap::TerrainTilemapPlugin));
    }
}

pub const TILES_PRE_CHUNK: UVec2 = uvec2(10, 10);
pub const TILE_COUNT: usize = TILES_PRE_CHUNK.x as usize * TILES_PRE_CHUNK.y as usize;
pub const TILE_SIZE: Vec2 = Vec2::new(
    Chunk::SIZE.x / TILES_PRE_CHUNK.x as f32,
    Chunk::SIZE.y / TILES_PRE_CHUNK.y as f32,
);

pub fn tile_index<T>(x: T, y: T) -> T
where
    T: From<u32> + Copy + Mul<Output = T> + Add<Output = T>,
{
    let tpc_x: T = T::from(TILES_PRE_CHUNK.x);
    y * tpc_x + x
}

#[derive(AssetCollection, Resource)]
pub struct TerrainTileAtlas {
    #[asset(path = "tile_map.png")]
    pub texture: Handle<Image>,
}
