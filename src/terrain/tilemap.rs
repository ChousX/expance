use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

use super::{TILE_SIZE, TILES_PRE_CHUNK, TerrainTileAtlas};
use crate::{
    chunk::{Chunk, ChunkPos},
    terrain::tile_data::{TerrainType, TileType},
};

pub struct TerrainTilemapPlugin;
impl Plugin for TerrainTilemapPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(add_tilemap_to_chunk);
    }
}

fn add_tilemap_to_chunk(
    trigger: Trigger<OnAdd, Chunk>,
    mut commands: Commands,
    chunks: Query<(&ChunkPos, &Transform)>,
    tile_map_atalas: Res<TerrainTileAtlas>,
) {
    let chunk_id = trigger.target();
    let Ok((chunk_pos, transform)) = chunks.get(chunk_id) else {
        warn!("chunk not found");
        return;
    };
    let mut tile_storage = TileStorage::empty(TILES_PRE_CHUNK.into());
    //build all tiles
    for x in 0..TILES_PRE_CHUNK.x {
        for y in 0..TILES_PRE_CHUNK.y {
            let tile_type = TileType::generate(x, y, **chunk_pos);
            let terrain_type = TerrainType::generate(x, y, **chunk_pos);

            let texture_index = TileTextureIndex(tile_type.get_texture_index());
            let color = TileColor(terrain_type.get_color());
            let tile_pos = TilePos { x, y };
            let tile_entity = commands
                .spawn((
                    TileBundle {
                        position: tile_pos,
                        tilemap_id: TilemapId(chunk_id),
                        texture_index,
                        color,
                        ..Default::default()
                    },
                    tile_type,
                    terrain_type,
                ))
                .id();
            commands.entity(chunk_id).add_child(tile_entity);
            tile_storage.set(&tile_pos, tile_entity);
        }
    }
    //Insert Tilemap into Chunk
    commands.entity(chunk_id).insert(TilemapBundle {
        grid_size: TILE_SIZE.into(),
        map_type: TilemapType::Square,
        size: TILES_PRE_CHUNK.into(),
        storage: tile_storage,
        texture: TilemapTexture::Single(tile_map_atalas.texture.clone()),
        tile_size: TILE_SIZE.into(),
        transform: *transform,
        anchor: TilemapAnchor::BottomLeft,
        render_settings: TilemapRenderSettings {
            render_chunk_size: TILES_PRE_CHUNK,
            ..Default::default()
        },
        ..Default::default()
    });
}
