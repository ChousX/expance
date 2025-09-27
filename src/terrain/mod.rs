use bevy::prelude::*;
use bevy_asset_loader::asset_collection::AssetCollection;

mod tile_data;
mod tilemap;

pub struct TerrainPlugin;
impl Plugin for TerrainPlugin {
    fn build(&self, app: &mut App) {}
}

pub const TILES_PRE_CHUNK: UVec2 = uvec2(10, 10);
pub const TILE_COUNT: usize = TILES_PRE_CHUNK.x as usize * TILES_PRE_CHUNK.y as usize;
pub const fn tile_index(x: usize, y: usize) -> usize {
    y * TILES_PRE_CHUNK.x as usize + x
}

#[derive(AssetCollection, Resource)]
pub struct TerrainTileAtlas {
    #[asset(path = "tile_map.png")]
    pub texture: Handle<Image>,
}
