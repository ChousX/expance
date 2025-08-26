use std::collections::HashMap;

use bevy::{
    ecs::query::{QueryData, QueryFilter},
    prelude::*,
};
use strum::{IntoStaticStr, VariantArray};

use crate::app::AppUpdate;

pub struct ChunkPlugin;
impl Plugin for ChunkPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ChunkManager>();
        app.add_observer(add_chunk_manager)
            .add_observer(remove_chunk_manager)
            .add_systems(
                Update,
                load_chunks_around_chunk_loader.in_set(AppUpdate::PostAction),
            )
            .add_observer(add_chunk_transform);
        #[cfg(feature = "chunk_info")]
        app.add_observer(show_chunk_spawn);
    }
}

#[cfg(feature = "chunk_info")]
fn show_chunk_spawn(trigger: Trigger<OnAdd, Chunk>, q: Query<(&ChunkLoadLevel, &ChunkPos)>) {
    let id = trigger.target();
    let Ok((load_level, pos)) = q.get(id) else {
        warn!("info was not there");
        return;
    };
    let str_ll : &str = load_level.into();
    info!("Chunk(id:{},load_level:{},pos:{})", id, str_ll, pos.0);
}

#[derive(Component)]
#[require(ChunkPos)]
pub struct Chunk;
impl Chunk {
    pub const SIZE: Vec2 = vec2(10.0, 10.0);
    pub fn transform_to_chunk_pos(transform: &Transform) -> IVec2 {
        let pos = transform.translation.xy();
        let Vec2 { x, y } = pos / Self::SIZE;
        ivec2(x as i32, y as i32)
    }
    pub fn g_transform_to_chunk_pos(transform: &GlobalTransform) -> IVec2 {
        let pos = transform.translation().xy();
        let Vec2 { x, y } = pos / Self::SIZE;
        ivec2(x as i32, y as i32)
    }
}

#[derive(Component, Default, Deref, DerefMut)]
pub struct ChunkPos(pub IVec3);
impl ChunkPos {
    pub fn into_vec3(&self) -> Vec3 {
        self.0.as_vec3() * Chunk::SIZE.extend(1.0)
    }
}

#[derive(VariantArray, Debug, PartialEq, Eq, Component, Default, Clone, Copy, IntoStaticStr)]
pub enum ChunkLoadLevel {
    Full,
    Mostly,
    #[default]
    Minimum,
}

#[derive(Resource, Default, Deref, DerefMut)]
pub struct ChunkManager(pub Vec<HashMap<IVec2, Entity>>);

impl ChunkManager {
    pub fn get(&self, pos: IVec3) -> Option<Entity> {
        let IVec3 { x, y, z } = pos;
        let level = self.get_level(z as usize)?;
        level.get(&ivec2(x, y)).copied()
    }

    pub fn insert(&mut self, pos: IVec3, id: Entity) -> Option<Entity> {
        let IVec3 { x, y, z } = pos;
        let level = self.get_level_mut(z as usize)?;
        level.insert(ivec2(x, y), id)
    }

    pub fn remove(&mut self, pos: IVec3) -> Option<Entity> {
        let IVec3 { x, y, z } = pos;
        let level = self.get_level_mut(z as usize)?;
        level.remove(&ivec2(x, y))
    }

    pub fn is_loaded(&self, pos: IVec3) -> bool {
        let IVec3 { x, y, z } = pos;
        let Some(level) = self.get_level(z as usize) else {
            return false;
        };
        level.contains_key(&ivec2(x, y))
    }

    pub fn get_level(&self, level: usize) -> Option<&HashMap<IVec2, Entity>> {
        self.0.get(level)
    }

    fn get_level_mut(&mut self, level: usize) -> Option<&mut HashMap<IVec2, Entity>> {
        self.0.get_mut(level)
    }
}

pub struct ChunkGrabber<'a, B: QueryData, C: QueryFilter> {
    chunks: Query<'a, 'a, B, C>,
    manger: &'a ChunkManager,
}

impl<'a, B: QueryData, C: QueryFilter> ChunkGrabber<'a, B, C> {
    pub fn new(chunks: Query<'a, 'a, B, C>, manger: &'a ChunkManager) -> Self {
        Self { chunks, manger }
    }

    pub fn get_chunk_at(
        &self,
        pos: IVec3,
    ) -> Option<<<B as QueryData>::ReadOnly as QueryData>::Item<'_>> {
        let id = self.manger.get(pos)?;
        self.chunks.get(id).ok()
    }
}

pub struct ChunkGrabberMut<'a, 'b, B: QueryData, C: QueryFilter> {
    chunks: Query<'a, 'a, B, C>,
    manger: &'b ChunkManager,
}

impl<'a, 'b, B: QueryData, C: QueryFilter> ChunkGrabberMut<'a, 'b, B, C> {
    pub fn new(chunks: Query<'a, 'a, B, C>, manger: &'b ChunkManager) -> Self {
        Self { chunks, manger }
    }

    pub fn get_chunk_at(&mut self, pos: IVec3) -> Option<<B as QueryData>::Item<'_>> {
        let id = self.manger.get(pos)?;
        self.chunks.get_mut(id).ok()
    }
}

fn add_chunk_manager(
    trigger: Trigger<OnAdd, Chunk>,
    mut chunk_manager: ResMut<ChunkManager>,
    chunks: Query<&ChunkPos>,
) {
    let chunk_id = trigger.target();
    let &ChunkPos(chunk_pos) = chunks.get(chunk_id).unwrap();
    if let Some(pre_chunk) = chunk_manager.insert(chunk_pos, chunk_id) {
        warn!("pushed out chunk:{pre_chunk}");
    }
}

fn remove_chunk_manager(
    trigger: Trigger<OnRemove, Chunk>,
    mut chunk_manager: ResMut<ChunkManager>,
    chunks: Query<&ChunkPos>,
) {
    let chunk_id = trigger.target();
    let ChunkPos(chunk_pos) = chunks.get(chunk_id).unwrap();
    if chunk_manager.remove(*chunk_pos).is_none() {
        warn!("No Chunk to remove at:{chunk_pos}");
    }
}

#[derive(Component, Default)]
#[require(Transform)]
pub struct ChunkLoader {
    pub full: Vec2,
    pub mostly: Vec2,
    pub minimum: Vec2,
}

impl ChunkLoader {
    pub fn chunk_pos_in_range(&self, pos: Vec2, load_level: ChunkLoadLevel) -> PointsInRange {
        let (target, min) = match load_level {
            ChunkLoadLevel::Full => (self.full, vec2(0.0, 0.0)),
            ChunkLoadLevel::Mostly => (self.mostly, self.full),
            ChunkLoadLevel::Minimum => (self.minimum, self.mostly),
        };
        let start_count = ((min + min) / Chunk::SIZE).as_ivec2();
        let stop_count = ((target + target) / Chunk::SIZE).as_ivec2();
        let chunk_num = stop_count - start_count;
        let chunk_pos = (pos / Chunk::SIZE).as_ivec2();

        let mut points = Vec::with_capacity(chunk_num.x as usize * chunk_num.y as usize);
        for y in start_count.y..stop_count.y {
            for x in start_count.x..stop_count.x {
                points.push(ivec2(x, y) + chunk_pos);
            }
        }
        PointsInRange(points)
    }
}

pub struct PointsInRange(Vec<IVec2>);
impl Iterator for PointsInRange {
    type Item = IVec2;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

#[derive(Component, Default)]
pub struct KeepChunkLoaded;

fn load_chunks_around_chunk_loader(
    chunk_loaders: Query<(&ChunkLoader, &Transform)>,
    chunk_manager: Res<ChunkManager>,
    mut commands: Commands,
) {
    for (loader, transform) in chunk_loaders.iter() {
        for load_level in ChunkLoadLevel::VARIANTS {
            for point in loader.chunk_pos_in_range(transform.translation.xy(), *load_level) {
                let chunk_pos = point.extend(transform.translation.z as i32);
                if !chunk_manager.is_loaded(chunk_pos) {
                    commands.spawn((Chunk, ChunkPos(chunk_pos), *load_level));
                }
            }
        }
    }
}

fn add_chunk_transform(
    trigger: Trigger<OnAdd, ChunkPos>,
    mut commands: Commands,
    chunk_q: Query<&ChunkPos>,
) {
    let id = trigger.target();
    let Ok(chunk_pos) = chunk_q.get(id) else {
        return;
    };
    commands
        .entity(id)
        .insert(Transform::from_translation(chunk_pos.into_vec3()));
}
