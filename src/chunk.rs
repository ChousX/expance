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
        app.init_resource::<ChunkManager>()
            .init_resource::<CurrentChunkLayer>();
        app.add_observer(add_chunk_manager)
            .add_observer(remove_chunk_manager)
            .add_systems(
                Update,
                load_chunks_around_chunk_loader.in_set(AppUpdate::PostAction),
            )
            .add_observer(add_chunk_transform)
            .add_systems(OnExit(AppState::Game), remove_chunk_loaders);
        #[cfg(feature = "chunk_info")]
        app.add_observer(show_chunk_spawn);
        app.init_state::<ShowChunkBounds>().add_systems(
            Update,
            draw_chunk_outlines.run_if(in_state(ShowChunkBounds::Yes)),
        );
    }
}

#[derive(States, Default, Clone, Eq, PartialEq, Hash, Debug, Copy)]
pub enum ShowChunkBounds {
    #[default]
    Yes,
    No,
}

#[cfg(feature = "chunk_info")]
fn show_chunk_spawn(trigger: Trigger<OnAdd, Chunk>, q: Query<(&LoadLevel, &ChunkPos)>) {
    let id = trigger.target();
    let Ok((load_level, pos)) = q.get(id) else {
        warn!("info was not there");
        return;
    };
    let str_ll: &str = load_level.into();
    info!("Chunk(id:{}, load_level:{},pos:{})", id, str_ll, pos.0);
}

#[derive(Component)]
#[require(ChunkPos, LoadLevel)]
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
#[component(immutable)]
pub enum LoadLevel {
    Full = 2,
    Mostly = 1,
    #[default]
    Minimum = 0,
}

#[derive(Resource, Default, Deref, DerefMut)]
pub struct ChunkManager(pub HashMap<i32, HashMap<IVec2, Entity>>);

impl ChunkManager {
    //Returns the chunk id is it exists
    pub fn get(&self, pos: IVec3) -> Option<Entity> {
        let IVec3 { x, y, z } = pos;
        self.0.get(&z)?.get(&ivec2(x, y)).copied()
    }

    //Inserts a new chunk into manager
    //Returns the previous chunk id if there was one
    pub fn insert(&mut self, pos: IVec3, id: Entity) -> Option<Entity> {
        let IVec3 { x, y, z } = pos;
        //the or_default seems wrong to me
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

#[derive(Resource, Default, Deref, DerefMut)]
pub struct CurrentChunkLayer(i32);

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
    pub full: IVec2,
    pub mostly: IVec2,
    pub minimum: IVec2,
}

#[derive(Component, Default)]
pub struct KeepChunkLoaded;

fn load_chunks_around_chunk_loader(
    chunk_loaders: Query<(&ChunkLoader, &GlobalTransform)>,
    chunk_manager: Res<ChunkManager>,
    chunk_load_levels: Query<&LoadLevel>,
    current_chunk_level: Res<CurrentChunkLayer>,
    mut commands: Commands,
) {
    // helper function
    fn aux(
        commands: &mut Commands,
        chunk_manager: &Res<ChunkManager>,
        chunk_load_levels: &Query<&LoadLevel>,
        current_chunk_level: &Res<CurrentChunkLayer>,
        load_level: LoadLevel,
        iter: impl Iterator<Item = IVec2>,
    ) {
        for point in iter {
            let chunk_id = point.extend(***current_chunk_level);
            if let Some(id) = chunk_manager.get(chunk_id) {
                //The chunk already exists so we need to check if its load level is correct
                let load_level = chunk_load_levels.get(id).expect("Chunk not found???");
                //if the chunk load level is too low, raise it
                if *load_level < *load_level {
                    commands.entity(id).insert(*load_level);
                }
            } else {
                //The chunk needs to be spawned
                commands.spawn((Chunk, ChunkPos(chunk_id), load_level));
            }
        }
    }
    for (loader_rangers, transform) in chunk_loaders.iter() {
        let &ChunkLoader {
            full,
            mostly,
            minimum,
        } = loader_rangers;
        let loader_pos = Chunk::g_transform_to_chunk_pos(transform);

        // get all point iters
        let minimum_level = shell_range(minimum, mostly, loader_pos);
        let mostly_level = shell_range(mostly, full, loader_pos);
        let full_level =
            (-full.x..=full.x).flat_map(move |x| (-full.y..=full.y).map(move |y| ivec2(x, y)));

        //update load levels or create chunks
        aux(
            &mut commands,
            &chunk_manager,
            &chunk_load_levels,
            &current_chunk_level,
            LoadLevel::Full,
            full_level,
        );
        aux(
            &mut commands,
            &chunk_manager,
            &chunk_load_levels,
            &current_chunk_level,
            LoadLevel::Mostly,
            mostly_level,
        );
        aux(
            &mut commands,
            &chunk_manager,
            &chunk_load_levels,
            &current_chunk_level,
            LoadLevel::Minimum,
            minimum_level,
        );
    }
}

//returns an iter of all points in outer but not in inner
fn shell_range(outer: IVec2, inner: IVec2, center_pos: IVec2) -> impl Iterator<Item = IVec2> {
    let outer_max = outer + center_pos;
    let outer_min = -outer + center_pos;
    let inner_max = inner + center_pos;
    let inner_min = -inner + center_pos;
    (outer_min.x..=outer_max.x)
        .flat_map(move |x| (outer_min.y..=outer_max.y).map(move |y| ivec2(x, y)))
        .filter_map(move |point| {
            if point.x >= inner_min.x
                && point.x <= inner_max.x
                && point.y >= inner_min.y
                && point.y <= inner_max.y
            {
                None
            } else {
                Some(point)
            }
        })
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_shell_range() {
        let result: Vec<IVec2> = shell_range(ivec2(2, 2), ivec2(1, 1), ivec2(0, 0)).collect();
        let existing = [
            ivec2(-2, -2),
            ivec2(-2, -1),
            ivec2(-2, 0),
            ivec2(-2, 1),
            ivec2(-2, 2),
            ivec2(-1, -2),
            ivec2(-1, 2),
            ivec2(0, -2),
            ivec2(0, 2),
            ivec2(1, -2),
            ivec2(1, 2),
            ivec2(2, -2),
            ivec2(2, -1),
            ivec2(2, 0),
            ivec2(2, 1),
            ivec2(2, 2),
        ];
        for point in existing {
            assert!(result.contains(&point));
        }
        let not_existing = [
            ivec2(-1, -1),
            ivec2(-1, 0),
            ivec2(-1, 1),
            ivec2(0, -1),
            ivec2(0, 0),
            ivec2(0, 1),
            ivec2(1, -1),
            ivec2(1, 0),
            ivec2(1, 1),
            ivec2(3, 3),
        ];
        for point in not_existing {
            assert!(!result.contains(&point));
        }
    }
}
