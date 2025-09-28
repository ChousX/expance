use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

use super::{
    TILES_PRE_CHUNK, TerrainTileAtlas,
    tile_data::{TerrainData, TileData},
};
use crate::chunk::{Chunk, LoadLevel};

pub struct TerrainTilemapPlugin;
impl Plugin for TerrainTilemapPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(add_tilemap_to_chunk);
        app.add_systems(Update, update_tilemap);
    }
}

fn add_tilemap_to_chunk(
    trigger: Trigger<OnInsert, LoadLevel>,
    chunks: Query<(&LoadLevel, Option<&TileStorage>, &TileData, &TerrainData)>,
    mut commands: Commands,
    tile_map_atalas: Res<TerrainTileAtlas>,
) {
    //Check if the load level is == to LoadLevel::Full
    //This will also filter out any chunks missing required data (don't known how they could be
    //missing data but yeah)
    let Ok((load_level, tile_storage, _tile_data, _terrain_data)) = chunks.get(trigger.target())
    else {
        return;
    };
    if load_level < &LoadLevel::Full {
        return;
    }
    //Check if the Tilemap already exists
    if tile_storage.is_none() {
        return;
    }
    //Init Tilemap
    //Build Tilemap
    let tile_size = Chunk::SIZE / TILES_PRE_CHUNK.as_vec2();
    let tilemap_entity = trigger.target();
    let mut tile_storage = TileStorage::empty(TILES_PRE_CHUNK.into());
    for x in 0..TILES_PRE_CHUNK.x {
        for y in 0..TILES_PRE_CHUNK.y {
            //TODO: update this use tile data and terrain date
            let tile_pos = TilePos { x, y };
            let tile_entity = commands
                .spawn(TileBundle {
                    position: tile_pos,
                    tilemap_id: TilemapId(tilemap_entity),
                    texture_index: TileTextureIndex((x + y) % 4),
                    ..Default::default()
                })
                .id();
            commands.entity(tilemap_entity).add_child(tile_entity);
            tile_storage.set(&tile_pos, tile_entity);
        }
    }

    //Insert Tilemap into Chunk
    commands.entity(tilemap_entity).insert(TilemapBundle {
        grid_size: tile_size.into(),
        map_type: TilemapType::Square,
        size: TILES_PRE_CHUNK.into(),
        storage: tile_storage,
        texture: TilemapTexture::Single(tile_map_atalas.texture.clone()),
        tile_size: tile_size.into(),
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        anchor: TilemapAnchor::BottomLeft,
        render_settings: TilemapRenderSettings {
            render_chunk_size: TILES_PRE_CHUNK,
            ..Default::default()
        },
        ..Default::default()
    });
}

fn update_tilemap(chunks: Query<(&TileStorage), Changed<LoadLevel>>) {
    //Update Tiles to reflect terrain data
}
