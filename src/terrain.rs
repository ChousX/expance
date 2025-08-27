use bevy::prelude::*;
use bevy_ecs_tilemap::{
    TilemapBundle,
    map::{TilemapId, TilemapRenderSettings, TilemapTexture},
    tiles::{TileBundle, TilePos, TileStorage},
};

use crate::chunk::Chunk;

pub struct TerrainPlugin;
impl Plugin for TerrainPlugin {
    fn build(&self, app: &mut App) {}
}

pub const TILES_PRE_CHUNK: UVec2 = uvec2(10, 10);

fn spawn_chunk(
    trigger: Trigger<OnAdd, Chunk>,
    commands: &mut Commands,
    asset_server: &AssetServer,
) {
    let tile_size = Chunk::SIZE / TILES_PRE_CHUNK.as_vec2();
    let tilemap_entity = trigger.target();
    let mut tile_storage = TileStorage::empty(TILES_PRE_CHUNK.into());
    // Spawn the elements of the tilemap.
    for x in 0..TILES_PRE_CHUNK.x {
        for y in 0..TILES_PRE_CHUNK.y {
            let tile_pos = TilePos { x, y };
            let tile_entity = commands
                .spawn(TileBundle {
                    position: tile_pos,
                    tilemap_id: TilemapId(tilemap_entity),
                    ..Default::default()
                })
                .id();
            commands.entity(tilemap_entity).add_child(tile_entity);
            tile_storage.set(&tile_pos, tile_entity);
        }
    }

    //todo add texture
    commands.entity(tilemap_entity).insert(TilemapBundle {
        grid_size: tile_size.into(),
        size: TILES_PRE_CHUNK.into(),
        storage: tile_storage,
        texture: TilemapTexture::Single(texture_handle),
        tile_size: tile_size.into(),
        render_settings: TilemapRenderSettings {
            render_chunk_size: TILES_PRE_CHUNK,
            ..Default::default()
        },
        ..Default::default()
    });
}
