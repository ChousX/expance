use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

use super::{
    TILES_PRE_CHUNK, TerrainTileAtlas,
    tile_data::{TerrainData, TileData},
};
use crate::{
    app::{AppState, AppUpdate},
    chunk::{Chunk, LoadLevel},
};

pub struct TerrainTilemapPlugin;
impl Plugin for TerrainTilemapPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(add_tilemap_to_chunk)
            .add_systems(Update, update_tilemap);
        app.add_event::<InsertTileMap>().add_systems(
            Update,
            insert_tilemap_to_chunk
                .in_set(AppUpdate::PostAction)
                .run_if(in_state(AppState::Game)),
        );
    }
}

#[derive(Event, Deref, DerefMut, Clone, Copy)]
pub struct InsertTileMap(pub Entity);

fn add_tilemap_to_chunk(
    trigger: Trigger<OnInsert, LoadLevel>,
    chunks: Query<(&LoadLevel, Option<&TileStorage>)>,
    mut output: EventWriter<InsertTileMap>,
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

fn update_tilemap(chunks: Query<(&TileStorage), Changed<LoadLevel>>) {
    //Update Tiles to reflect terrain data
}

fn insert_tilemap_to_chunk(
    mut commands: Commands,
    chunks: Query<(Entity, &TileData, &TerrainData, &Transform), Without<TileStorage>>,
    tile_map_atalas: Res<TerrainTileAtlas>,
) {
    for (tilemap_entity, _tile_date, _terrain_data, transform) in chunks.iter() {
        //Init Tilemap
        //Build Tilemap
        let tile_size = Chunk::SIZE / TILES_PRE_CHUNK.as_vec2();
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
