use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

use super::{TILE_SIZE, TILES_PRE_CHUNK, TerrainTileAtlas};
use crate::{
    app::{AppState, AppUpdate},
    chunk::{Chunk, LoadLevel},
};

pub struct TerrainTilemapPlugin;
impl Plugin for TerrainTilemapPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(add_tilemap_to_chunk);
        app.add_systems(
            Update,
            insert_tilemap_to_chunk
                .in_set(AppUpdate::PostAction)
                .run_if(in_state(AppState::Game)),
        );
    }
}

fn add_tilemap_to_chunk(
    trigger: Trigger<OnInsert, LoadLevel>,
    chunks: Query<(&LoadLevel, Option<&TileStorage>)>,
) {
    //Check if the load level is == to LoadLevel::Full
    let Ok((load_level, tile_storage)) = chunks.get(trigger.target()) else {
        return;
    };
    if load_level < &LoadLevel::Full {
        return;
    }
    //Check if the Tilemap already exists
    if tile_storage.is_some() {
        return;
    }
    //output.write(InsertTileMap(trigger.target()));
}

fn insert_tilemap_to_chunk(
    mut commands: Commands,
    chunks: Query<(Entity, &TileData, &TerrainData, &Transform), Without<TileStorage>>,
    tile_map_atalas: Res<TerrainTileAtlas>,
) {
    for (tilemap_entity, tile_date, terrain_data, transform) in chunks.iter() {
        //Init Tilemap
        //Build Tilemap
        let mut tile_storage = TileStorage::empty(TILES_PRE_CHUNK.into());
        for x in 0..TILES_PRE_CHUNK.x {
            for y in 0..TILES_PRE_CHUNK.y {
                //TODO: update this use terrain date
                let texture_index = TileTextureIndex(tile_date.get_texture_index(x, y));
                let color = TileColor(terrain_data.get_color(x, y));
                let tile_pos = TilePos { x, y };
                let tile_entity = commands
                    .spawn(TileBundle {
                        position: tile_pos,
                        tilemap_id: TilemapId(tilemap_entity),
                        texture_index,
                        color,
                        ..Default::default()
                    })
                    .id();
                commands.entity(tilemap_entity).add_child(tile_entity);
                tile_storage.set(&tile_pos, tile_entity);
            }
        }

        //Insert Tilemap into Chunk
        commands.entity(tilemap_entity).insert(TilemapBundle {
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
}
