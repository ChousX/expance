use std::collections::HashMap;
use std::collections::HashSet;

use bevy::{
    ecs::query::{QueryData, QueryFilter},
    prelude::*,
};
use strum::{IntoStaticStr, VariantArray};

use crate::app::AppState;
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
            .add_observer(add_chunk_transform)
            .add_systems(OnExit(AppState::Game), remove_chunk_loaders);
        #[cfg(feature = "chunk_info")]
        app.add_observer(show_chunk_spawn)
            .add_systems(Update, draw_chunk_outlines);
    }
}

#[cfg(feature = "chunk_info")]
fn show_chunk_spawn(trigger: Trigger<OnAdd, Chunk>, q: Query<(&LoadLevel, &ChunkPos)>) {
    let id = trigger.target();
    let Ok((load_level, pos)) = q.get(id) else {
        warn!("info was not there");
        return;
    };
    let str_ll: &str = load_level.into();
    info!("Chunk(id:{},load_level:{},pos:{})", id, str_ll, pos.0);
}

#[derive(Component)]
#[require(ChunkPos)]
pub struct Chunk;
impl Chunk {
    pub const SIZE: Vec2 = vec2(500.0, 500.0);
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

#[derive(
    PartialOrd,
    Ord,
    VariantArray,
    Debug,
    PartialEq,
    Eq,
    Component,
    Default,
    Clone,
    Copy,
    IntoStaticStr,
    Hash,
)]
pub enum LoadLevel {
    Full = 2,
    Mostly = 1,
    #[default]
    Minimum = 0,
}

#[derive(Resource, Default, Deref, DerefMut)]
pub struct ChunkManager(pub HashMap<i32, HashMap<IVec2, Entity>>);

impl ChunkManager {
    pub fn get(&self, pos: IVec3) -> Option<Entity> {
        let IVec3 { x, y, z } = pos;
        self.0.get(&z)?.get(&ivec2(x, y)).copied()
    }

    pub fn insert(&mut self, pos: IVec3, id: Entity) -> Option<Entity> {
        let IVec3 { x, y, z } = pos;
        self.0.entry(z).or_default().insert(ivec2(x, y), id)
    }

    pub fn remove(&mut self, pos: IVec3) -> Option<Entity> {
        let IVec3 { x, y, z } = pos;
        self.0.get_mut(&z)?.remove(&ivec2(x, y))
    }

    pub fn is_loaded(&self, pos: IVec3) -> bool {
        let IVec3 { x, y, z } = pos;
        self.0
            .get(&z)
            .map_or(false, |level| level.contains_key(&ivec2(x, y)))
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
    pub fn chunk_pos_in_range(&self, pos: Vec2, load_level: LoadLevel) -> PointsInRange {
        let (target, min) = match load_level {
            LoadLevel::Full => (self.full, vec2(0.0, 0.0)),
            LoadLevel::Mostly => (self.mostly, self.full),
            LoadLevel::Minimum => (self.minimum, self.mostly),
        };
        let min_pos = pos - target;
        let max_pos = pos + target;

        let min_chunk = (min_pos / Chunk::SIZE).floor().as_ivec2();
        let max_chunk = (max_pos / Chunk::SIZE).ceil().as_ivec2();

        let mut points = Vec::new();
        for y in min_chunk.y..max_chunk.y {
            for x in min_chunk.x..max_chunk.x {
                points.push(ivec2(x, y));
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
    existing_chunks: Query<&LoadLevel>,
) {
    let mut seen_chunks = HashSet::new();
    for (loader, transform) in chunk_loaders.iter() {
        let base_z = transform.translation.z as i32;

        for &load_level in LoadLevel::VARIANTS {
            for point in loader.chunk_pos_in_range(transform.translation.xy(), load_level) {
                let chunk_pos = point.extend(base_z);

                // Only spawn once per position
                if !seen_chunks.insert(chunk_pos) {
                    continue;
                }

                if let Some(existing_entity) = chunk_manager.get(chunk_pos) {
                    // Update the load level of the existing chunk
                    if let Ok(current_level) = existing_chunks.get(existing_entity) {
                        if *current_level < load_level {
                            commands.entity(existing_entity).insert(load_level);
                        }
                    }
                    continue;
                }

                let id = commands
                    .spawn((Chunk, ChunkPos(chunk_pos), load_level))
                    .id();

                #[cfg(feature = "chunk_info")]
                info!("new:{id:?}");
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
    info!("new:{id}");
}

fn draw_chunk_outlines(
    chunks: Query<(&ChunkPos, Option<&GlobalTransform>, &LoadLevel)>,
    mut gizmos: Gizmos,
) {
    for (chunk_pos, global_transform, load_level) in &chunks {
        let base_pos = chunk_pos.into_vec3();
        let world_pos = global_transform.map_or(base_pos, |g| g.translation());

        const SIZE: Vec2 = Chunk::SIZE;
        let bottom_left = world_pos;
        let bottom_right = bottom_left + Vec3::new(SIZE.x, 0.0, 0.0);
        let top_right = bottom_right + Vec3::new(0.0, SIZE.y, 0.0);
        let top_left = bottom_left + Vec3::new(0.0, SIZE.y, 0.0);

        let color = match load_level {
            LoadLevel::Full => bevy::color::palettes::tailwind::GREEN_500,
            LoadLevel::Mostly => bevy::color::palettes::tailwind::YELLOW_500,
            LoadLevel::Minimum => bevy::color::palettes::tailwind::RED_500,
        };

        gizmos.line(top_left, top_right, color);
        gizmos.line(top_right, bottom_right, color);
        gizmos.line(bottom_right, bottom_left, color);
        gizmos.line(bottom_left, top_left, color);
    }
}

fn remove_chunk_loaders(mut commands: Commands, chunk_loaders: Query<Entity, With<ChunkLoader>>) {
    for chunk_loader in chunk_loaders.iter() {
        commands.entity(chunk_loader).remove::<ChunkLoader>();
    }
}
