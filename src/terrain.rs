use bevy::prelude::*;
use bevy_asset_loader::asset_collection::AssetCollection;
use bevy_ecs_tilemap::{
    TilemapBundle,
    anchor::TilemapAnchor,
    map::{TilemapId, TilemapRenderSettings, TilemapTexture, TilemapType},
    tiles::{TileBundle, TilePos, TileStorage, TileTextureIndex},
};

use crate::chunk::{Chunk, LoadLevel};

pub struct TerrainPlugin;
impl Plugin for TerrainPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(add_tilemap_to_chunk);
    }
}

pub const TILES_PRE_CHUNK: UVec2 = uvec2(10, 10);

#[derive(AssetCollection, Resource)]
pub struct TerrainTileAtlas {
    #[asset(path = "tile_map.png")]
    pub texture: Handle<Image>,
}

fn add_tilemap_to_chunk(
    trigger: Trigger<OnInsert, LoadLevel>,
    mut commands: Commands,
    tile_map_atalas: Res<TerrainTileAtlas>,
    chunk: Query<(&LoadLevel, Option<&Children>)>,
    tilemaps: Query<Entity, With<TileStorage>>,
) {
    //Only Load Tilemaps on Full LoadLevel
    let Ok((LoadLevel::Full, kids)) = chunk.get(trigger.target()) else {
        return;
    };
    //if the tilemap already exist, stop now
    if let Some(kids) = kids {
        for kid in kids.iter() {
            if tilemaps.contains(kid) {
                return;
            }
        }
    }

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
                    texture_index: TileTextureIndex((x + y) % 4),
                    ..Default::default()
                })
                .id();
            commands.entity(tilemap_entity).add_child(tile_entity);
            tile_storage.set(&tile_pos, tile_entity);
        }
    }

    commands.entity(tilemap_entity).insert(TilemapBundle {
        grid_size: tile_size.into(),
        map_type: TilemapType::Square,
        size: TILES_PRE_CHUNK.into(),
        storage: tile_storage,
        texture: TilemapTexture::Single(tile_map_atalas.texture.clone()),
        tile_size: tile_size.into(),
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        anchor: TilemapAnchor::Center,
        render_settings: TilemapRenderSettings {
            render_chunk_size: TILES_PRE_CHUNK,
            ..Default::default()
        },
        ..Default::default()
    });
}
