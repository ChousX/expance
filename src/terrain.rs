use bevy::prelude::*;
use bevy_asset_loader::asset_collection::AssetCollection;
use bevy_ecs_tilemap::{
    TilemapBundle,
    map::{TilemapId, TilemapRenderSettings, TilemapTexture, TilemapType},
    tiles::{TileBundle, TilePos, TileStorage, TileTextureIndex},
};

use crate::chunk::Chunk;

pub struct TerrainPlugin;
impl Plugin for TerrainPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(add_terrain_to_chunk);
    }
}

pub const TILES_PRE_CHUNK: UVec2 = uvec2(10, 10);

#[derive(AssetCollection, Resource)]
pub struct TerrainTileAtlas {
    #[asset(path = "tile_map.png")]
    pub texture: Handle<Image>,
}

fn add_terrain_to_chunk(
    trigger: Trigger<OnAdd, Chunk>,
    mut commands: Commands,
    tile_map_atalas: Res<TerrainTileAtlas>,
) {
    info!("Spawn Terrain");
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
        render_settings: TilemapRenderSettings {
            render_chunk_size: TILES_PRE_CHUNK,
            ..Default::default()
        },
        ..Default::default()
    });
}
