use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

use super::{TILE_COUNT, TILE_SIZE, TILES_PRE_CHUNK};
use crate::{
    app::AppUpdate,
    chunk::{Chunk, ChunkManager, LoadLevel},
};

pub struct TerrainDataPlugin;
impl Plugin for TerrainDataPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(add_terrain_data_to_chunk);
        app.add_observer(add_tile_data_to_chunk);

        app.add_event::<BrakeTile>()
            .add_systems(
                Update,
                (brake_tile, sync_tile_texure_with_tile_data)
                    .chain()
                    .in_set(AppUpdate::PostAction),
            )
            .add_systems(Update, brake_tiles_around_pos.in_set(AppUpdate::Action));
    }
}

#[derive(Clone, Copy, Default)]
pub enum TileType {
    #[default]
    Wall,
    Ground,
}
impl TileType {
    pub fn get_texture_index(&self) -> u32 {
        match self {
            TileType::Wall => 1,
            TileType::Ground => 2,
        }
    }
}

#[derive(Component, Clone, Copy, Deref, DerefMut)]
pub struct TileData(pub [TileType; TILE_COUNT]);

impl TileData {
    pub fn get_texture_index(&self, x: u32, y: u32) -> u32 {
        let index = super::tile_index(x, y);
        self[index as usize].get_texture_index()
    }
}

impl Default for TileData {
    fn default() -> Self {
        Self([TileType::default(); TILE_COUNT])
    }
}

#[derive(Clone, Copy, Default)]
pub enum TerrainType {
    #[default]
    Stone,
    Dirt,
    Sand,
}

impl TerrainType {
    pub fn get_terrain_color(&self) -> Color {
        match self {
            TerrainType::Stone => Color::srgba(0.1, 0.1, 0.2, 0.5),
            TerrainType::Dirt => Color::srgba(0.3, 0.1, 0.1, 0.5),
            TerrainType::Sand => Color::srgba(0.3, 0.8, 0.2, 0.5),
        }
    }
}

#[derive(Component, Clone, Copy, Deref, DerefMut)]
pub struct TerrainData(pub [TerrainType; TILE_COUNT]);
impl Default for TerrainData {
    fn default() -> Self {
        Self([TerrainType::default(); TILE_COUNT])
    }
}
impl TerrainData {
    pub fn get_color(&self, x: u32, y: u32) -> Color {
        let index = super::tile_index(x, y);
        self[index as usize].get_terrain_color()
    }
}

fn add_terrain_data_to_chunk(
    trigger: Trigger<OnInsert, LoadLevel>,
    chunks: Query<&LoadLevel>,
    mut commands: Commands,
) {
    let Ok(load_level) = chunks.get(trigger.target()) else {
        return;
    };
    if load_level < &LoadLevel::Mostly {
        return;
    }
    commands
        .entity(trigger.target())
        .insert(TerrainData::default());
}

fn add_tile_data_to_chunk(
    trigger: Trigger<OnInsert, LoadLevel>,
    chunks: Query<&LoadLevel>,
    mut commands: Commands,
) {
    let Ok(load_level) = chunks.get(trigger.target()) else {
        return;
    };
    if load_level < &LoadLevel::Mostly {
        return;
    }
    commands
        .entity(trigger.target())
        .insert(TileData::default());
}

#[derive(Event, Clone, Copy)]
//I may just want to make this an enum and have two ways to send the event. By entity or by position
/// Transfom the NonGround into a Ground tile
pub enum BrakeTile {
    ByEntity(Entity),
    ByPos(Vec3),
}

fn brake_tile(
    mut events: EventReader<BrakeTile>,
    tiles: Query<(&ChildOf, &Transform)>,
    mut tile_data: Query<&mut TileData>,
    chunk_manager: Res<ChunkManager>,
) {
    for event in events.read() {
        match event {
            BrakeTile::ByEntity(tile) => {
                let Ok((child_of, transform)) = tiles.get(*tile) else {
                    warn!("Tile does not exist: {tile}");
                    continue;
                };
                let Ok(mut tile_data) = tile_data.get_mut(child_of.parent()) else {
                    warn!("could not access tile data: {tile}");
                    continue;
                };
                let local_tile_pos = super::get_local_tile_pos(transform.translation.xy());
                let local_tile_id = super::tile_index(local_tile_pos.x, local_tile_pos.y) as usize;
                tile_data[local_tile_id] = TileType::Ground;
            }
            BrakeTile::ByPos(pos) => {
                let Some(chunk) = chunk_manager.get(Chunk::get_chunk_pos(*pos)) else {
                    warn!("No chunk at pos: {pos}");
                    continue;
                };
                let local_tile_pos = super::get_local_tile_pos(pos.xy());
                let local_tile_id = super::tile_index(local_tile_pos.x, local_tile_pos.y) as usize;
                if let Ok(mut tile_data) = tile_data.get_mut(chunk) {
                    tile_data[local_tile_id] = TileType::Ground;
                } else {
                    warn!(
                        "could not access tile data at: {chunk}. local tile pos:{local_tile_pos}"
                    );
                }
            }
        }
    }
}

fn sync_tile_texure_with_tile_data(
    tile_data: Query<(&TileData, &TileStorage), Changed<TileData>>,
    mut tile_texure_index: Query<&mut TileTextureIndex>,
) {
    for (tile_data, tile_storage) in tile_data.iter() {
        for (index, tile_type) in tile_data.0.iter().enumerate() {
            let x = index as u32 % TILES_PRE_CHUNK.x;
            let y = index as u32 / TILES_PRE_CHUNK.x;
            let Some(tile_entity) = tile_storage.get(&TilePos { x, y }) else {
                warn!("No tile at {} {}", x, y);
                continue;
            };
            if let Ok(mut texture_index) = tile_texure_index.get_mut(tile_entity) {
                let tile_type_texure_index = tile_type.get_texture_index();
                if tile_type_texure_index != texture_index.0 {
                    texture_index.0 = tile_type_texure_index;
                }
            }
        }
    }
}

/// Marker component that need their surrounding tiles broken
#[derive(Component)]
pub struct NeedsTileBreaking;

fn brake_tiles_around_pos(
    mut commands: Commands,
    cores: Query<(Entity, &GlobalTransform), With<NeedsTileBreaking>>,
    mut out: EventWriter<BrakeTile>,
) {
    for (entity, transform) in cores.iter() {
        let translation = transform.translation();
        let offset = translation.xy();
        let z = translation.z;

        //Brake all tiles around pos
        for x in -1..=1 {
            for y in -1..=1 {
                out.write(BrakeTile::ByPos(vec3(
                    x as f32 * TILE_SIZE.x + offset.x,
                    y as f32 * TILE_SIZE.y + offset.y,
                    z,
                )));
            }
        }

        // Remove the marker component so we don't process this core again
        commands.entity(entity).remove::<NeedsTileBreaking>();
    }
}
