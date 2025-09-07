use bevy::prelude::*;
use bevy_asset_loader::asset_collection::AssetCollection;
use bevy_ecs_tilemap::{
    TilemapBundle,
    map::{TilemapId, TilemapRenderSettings, TilemapTexture},
    tiles::{TileBundle, TilePos, TileStorage},
};

use crate::{app::AppLoadingState, chunk::Chunk};

pub struct TerrainPlugin;
impl Plugin for TerrainPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(AppLoadingState::Loaded),
            transform_raw_tile_textures_to_atlas,
        );
    }
}

pub const TILES_PRE_CHUNK: UVec2 = uvec2(10, 10);

#[derive(AssetCollection, Resource)]
pub struct RawTileTextures {
    #[asset(path = "tile_map_atalas", collection(typed))]
    tiles: Vec<Handle<Image>>,
}

#[derive(Resource)]
pub struct TerrainTileAtlas {
    pub texture: Handle<Image>,
    pub layout: Handle<TextureAtlasLayout>,
}

///Builds TerrainTileAtuas from loaded RawTileTextures and removes RawTileTextures when done
fn transform_raw_tile_textures_to_atlas(
    mut commands: Commands,
    raw_tile_textures: Res<RawTileTextures>,
    mut images: ResMut<Assets<Image>>,
    mut layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let mut texture_atlas_builder = TextureAtlasBuilder::default();
    for handle in raw_tile_textures.tiles.iter() {
        let Some(texture) = images.get(handle) else {
            continue;
        };
        texture_atlas_builder.add_texture(Some(handle.id()), texture);
    }

    let (texture_atlas_layout, _texture_atlas_sources, texture) =
        texture_atlas_builder.build().unwrap();
    let texture = images.add(texture);
    let layout = layouts.add(texture_atlas_layout);
    let terrain_tile_atlas = TerrainTileAtlas { texture, layout };
    commands.insert_resource(terrain_tile_atlas);
    commands.remove_resource::<RawTileTextures>();
}

fn spawn_chunk(
    trigger: Trigger<OnAdd, Chunk>,
    commands: &mut Commands,
    tile_map_atalas: Res<TerrainTileAtlas>,
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
        texture: TilemapTexture::Single(tile_map_atalas.texture.clone()),
        tile_size: tile_size.into(),
        render_settings: TilemapRenderSettings {
            render_chunk_size: TILES_PRE_CHUNK,
            ..Default::default()
        },
        ..Default::default()
    });
}
