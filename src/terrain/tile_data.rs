use bevy::{
    ecs::{component::HookContext, world::DeferredWorld},
    prelude::*,
};
use bevy_ecs_tilemap::prelude::*;

use super::TILE_SIZE;
use crate::{
    app::AppUpdate,
    chunk::{Chunk, ChunkManager},
};

pub struct TerrainDataPlugin;
impl Plugin for TerrainDataPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<BrakeTile>()
            .add_systems(Update, brake_tile.in_set(AppUpdate::PostAction));
    }
}
#[derive(Clone, Copy, Default, Component)]
#[require(TileTextureIndex)]
#[component(
    immutable,
    on_replace = on_tile_type_replace,
)]
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

    pub fn generate(x: u32, y: u32, chunk_pos: IVec3) -> Self {
        Self::Wall
    }
}

fn on_tile_type_replace(mut world: DeferredWorld, HookContext { entity, .. }: HookContext) {
    let texture_index = world.get::<TileType>(entity).unwrap().get_texture_index();
    if texture_index == 2 {
        info!("texture index {texture_index}");
    }
    world
        .commands()
        .entity(entity)
        .insert(TileTextureIndex(texture_index));
}

#[derive(Clone, Copy, Default, Component)]
#[require(TileColor)]
#[component(
    immutable,
    on_replace = on_terrain_type_replace,
)]
pub enum TerrainType {
    #[default]
    Stone,
    Dirt,
    Sand,
}

impl TerrainType {
    pub fn get_color(&self) -> Color {
        match self {
            TerrainType::Stone => Color::srgba(0.1, 0.1, 0.2, 0.5),
            TerrainType::Dirt => Color::srgba(0.3, 0.1, 0.1, 0.5),
            TerrainType::Sand => Color::srgba(0.3, 0.8, 0.2, 0.5),
        }
    }

    pub fn generate(x: u32, y: u32, chunk_pos: IVec3) -> Self {
        TerrainType::Sand
    }
}

fn on_terrain_type_replace(mut world: DeferredWorld, HookContext { entity, .. }: HookContext) {
    let terrain_color = world.get::<TerrainType>(entity).unwrap().get_color();
    world.get_mut::<TileColor>(entity).unwrap().0 = terrain_color;
}

#[derive(Event, Clone, Copy)]
pub enum BrakeTile {
    ByEntity(Entity),
    ByPos(Vec3),
}

fn brake_tile(
    mut events: EventReader<BrakeTile>,
    chunks: Query<&TileStorage>,
    chunk_manager: Res<ChunkManager>,
    mut commands: Commands,
) {
    for event in events.read() {
        match event {
            BrakeTile::ByEntity(tile) => {
                commands.entity(*tile).insert(TileType::Ground);
            }
            BrakeTile::ByPos(pos) => {
                //info!("Tile brake at {pos}");
                let Some(chunk_id) = chunk_manager.get_chunk_at(pos) else {
                    warn!("no chunk at pos:{pos}");
                    continue;
                };
                let Ok(tile_storage) = chunks.get(chunk_id) else {
                    warn!("chunk has no tilestorage:{chunk_id}");
                    continue;
                };
                let tile_index = get_tile_chunk_index(pos.xy());
                let Some(tile_id) = tile_storage.get(&TilePos::from(tile_index)) else {
                    warn!("no tile at pos:{tile_index}");
                    continue;
                };
                commands.entity(tile_id).insert(TileType::Ground);
            }
        }
    }
}

//Think this should work if there are picking errors check here first
fn get_tile_chunk_index(pos: Vec2) -> UVec2 {
    let mut local_pos = pos % Chunk::SIZE;
    if local_pos.x.is_sign_negative() {
        local_pos.x = Chunk::SIZE.x + local_pos.x;
    }
    if local_pos.y.is_sign_negative() {
        local_pos.y = Chunk::SIZE.y + local_pos.y;
    }
    (local_pos / TILE_SIZE).as_uvec2()
}

///Brake all tiles around point by the range.
pub fn brake_all_tiles_around(point: Vec3, range: u32, out: &mut EventWriter<BrakeTile>) {
    let range = range as i32;
    for x in -range..=range {
        for y in -range..=range {
            out.write(BrakeTile::ByPos(
                point + Vec3::new(x as f32, y as f32, 0.0) * TILE_SIZE.extend(1.0),
            ));
        }
    }
}
