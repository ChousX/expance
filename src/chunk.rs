use std::collections::HashMap;

use bevy::{
    ecs::{
        component::HookContext,
        query::{QueryData, QueryFilter},
        world::DeferredWorld,
    },
    prelude::*,
};

use crate::app::AppUpdate;

pub struct ChunkPlugin;
impl Plugin for ChunkPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ChunkManager>()
            .init_resource::<CurrentChunkLayer>();

        app.add_systems(
            Update,
            load_chunks_around_chunk_loader.in_set(AppUpdate::Action),
        );
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
fn show_chunk_spawn(trigger: Trigger<OnAdd, Chunk>, q: Query<&ChunkPos>) {
    let id = trigger.target();
    let Ok(pos) = q.get(id) else {
        warn!("info was not there");
        return;
    };
    info!("Chunk(id:{}, pos:{})", id, pos.0);
}

#[derive(Component)]
#[require(ChunkPos)]
#[component(
    immutable,
    on_add= on_add_chunk,
    on_remove = on_remove_chunk
)]
pub struct Chunk;
impl Chunk {
    pub const SIZE: Vec2 = vec2(500.0, 500.0);
    pub fn transform_to_chunk_pos(transform: &Transform) -> IVec3 {
        let pos = transform.translation;
        Self::get_chunk_pos(pos)
    }
    pub fn g_transform_to_chunk_pos(transform: &GlobalTransform) -> IVec3 {
        let pos = transform.translation();
        Self::get_chunk_pos(pos)
    }
    pub fn get_chunk_pos(pos: Vec3) -> IVec3 {
        let Vec2 { x, y } = pos.xy() / Self::SIZE;

        ivec3(x as i32, y as i32, pos.z as i32)
    }
}

///Adds Chunk to ChunkManager
fn on_add_chunk(mut world: DeferredWorld, HookContext { entity, .. }: HookContext) {
    let chunk_pos = world.get::<ChunkPos>(entity).unwrap().0;
    world
        .get_resource_mut::<ChunkManager>()
        .unwrap()
        .insert(chunk_pos, entity);
}

///Removes Chunk from ChunkManager
fn on_remove_chunk(mut world: DeferredWorld, HookContext { entity, .. }: HookContext) {
    let chunk_pos = world.get::<ChunkPos>(entity).unwrap().0;
    world
        .get_resource_mut::<ChunkManager>()
        .unwrap()
        .remove(chunk_pos);
}

#[derive(Component, Default, Deref, DerefMut)]
#[require(Transform)]
#[component(
    immutable,
    on_add= on_add_chunk_pos,
)]
pub struct ChunkPos(pub IVec3);
impl ChunkPos {
    pub fn into_vec3(&self) -> Vec3 {
        self.0.as_vec3() * Chunk::SIZE.extend(1.0)
    }
}

///Updates Transform to match ChunkPos
fn on_add_chunk_pos(mut world: DeferredWorld, HookContext { entity, .. }: HookContext) {
    let translation = world.get::<ChunkPos>(entity).unwrap().into_vec3();
     world.get_mut::<Transform>(entity).unwrap().translation = translation;
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

#[derive(Component, Default)]
#[require(Transform)]
pub struct ChunkLoader(pub IVec2);

#[derive(Component, Default)]
pub struct KeepChunkLoaded;

fn load_chunks_around_chunk_loader(
    chunk_loaders: Query<(&ChunkLoader, &GlobalTransform)>,
    chunk_manager: Res<ChunkManager>,
    current_chunk_layer: Res<CurrentChunkLayer>,
    mut commands: Commands,
) {
    for (loader_rangers, transform) in chunk_loaders.iter() {
        let &ChunkLoader(range) = loader_rangers;
        let loader_pos = Chunk::g_transform_to_chunk_pos(transform).xy();

        // get all point iters
        let iter = (-range.x..=range.x)
            .flat_map(move |x| (-range.y..=range.y).map(move |y| ivec2(x, y) + loader_pos));
        //Check if chunk is
        for point in iter {
            let chunk_id = point.extend(**current_chunk_layer);
            if let None = chunk_manager.get(chunk_id) {
                //The chunk needs to be spawned
                commands.spawn((Chunk, ChunkPos(chunk_id)));
            }
        }
    }
}

//returns an iter of all points in outer but not in inner
fn _shell_range(outer: IVec2, inner: IVec2, center_pos: IVec2) -> impl Iterator<Item = IVec2> {
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

fn draw_chunk_outlines(chunks: Query<(&ChunkPos, Option<&GlobalTransform>)>, mut gizmos: Gizmos) {
    for (chunk_pos, global_transform) in &chunks {
        let base_pos = chunk_pos.into_vec3();
        let world_pos = global_transform.map_or(base_pos, |g| g.translation());

        const SIZE: Vec2 = Chunk::SIZE;

        let bottom_left = world_pos;
        let bottom_right = bottom_left + Vec3::new(SIZE.x, 0.0, 0.0);
        let top_right = bottom_right + Vec3::new(0.0, SIZE.y, 0.0);
        let top_left = bottom_left + Vec3::new(0.0, SIZE.y, 0.0);

        let color = bevy::color::palettes::tailwind::GREEN_500;

        gizmos.line(top_left, top_right, color);
        gizmos.line(top_right, bottom_right, color);
        gizmos.line(bottom_right, bottom_left, color);
        gizmos.line(bottom_left, top_left, color);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_shell_range() {
        let result: Vec<IVec2> = _shell_range(ivec2(2, 2), ivec2(1, 1), ivec2(0, 0)).collect();
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
