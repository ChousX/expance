use bevy::prelude::*;

use crate::app::AppUpdate;

pub struct MoveEntityToPlugin;
impl Plugin for MoveEntityToPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, move_entity.in_set(AppUpdate::Action));
    }
}

/// Component that moves an entity from `from` to `to` using an easing curve
#[derive(Component)]
#[require(Speed)]
pub struct MoveEntityTo {
    pub to: Vec2,
    pub from: Vec2,
    pub easing: EaseFunction,
}

#[derive(Component, Deref, DerefMut)]
pub struct Speed(pub f32);
impl Speed {
    pub const DEFAULT: f32 = 100.0;
}
impl Default for Speed {
    fn default() -> Self {
        Self(Self::DEFAULT)
    }
}

fn move_entity(
    time: Res<Time>,
    mut commands: Commands,
    mut query: Query<(
        Entity,
        &mut Transform,
        &GlobalTransform,
        &MoveEntityTo,
        &Speed,
    )>,
) {
    for (entity, mut transform, g_transform, move_to, Speed(speed)) in query.iter_mut() {
        let current_pos = g_transform.translation().xy();
        let MoveEntityTo { to, from, easing } = *move_to;
        let total_distance = from.distance(to);
        // Handle edge case where from == to
        if total_distance <= f32::EPSILON {
            transform.translation = to.extend(transform.translation.z);
            commands.entity(entity).remove::<MoveEntityTo>();
            continue;
        }
        let traveled_distance = from.distance(current_pos);
        let remaining_distance = current_pos.distance(to);
        // Calculate progress (0.0 to 1.0)
        let t = (traveled_distance / total_distance).min(1.0);
        let Some(eased_t) = easing.sample(t) else {
            warn!("easing sample({t}) failed");
            continue;
        };
        // Calculate movement step
        let direction = (to - current_pos).normalize_or_zero();
        let base_move_distance = speed * time.delta_secs();

        // Apply easing to movement speed
        // Use the easing value directly, but ensure minimum movement
        let easing_speed_factor = if t >= 1.0 {
            1.0
        } else {
            // For most easing functions, we want some minimum speed even when eased_t is 0
            // This prevents the entity from getting stuck at the start
            (eased_t + 0.1).min(1.0)
        };
        let move_step = direction * base_move_distance * easing_speed_factor;
        // Check if we'll overshoot the target
        if remaining_distance <= move_step.length() {
            // Snap to target
            transform.translation = to.extend(transform.translation.z);
            commands.entity(entity).remove::<MoveEntityTo>();
            //info!("Reached target for entity {:?}", entity);
        } else {
            // Apply movement
            transform.translation += move_step.extend(0.0);
        }
    }
}

#[derive(Component, Default)]
pub struct Halt;

pub fn on_halt<T>(trigger: Trigger<OnAdd, Halt>, mut commands: Commands, query: Query<&T>)
where
    T: Component,
{
    if query.get(trigger.target()).is_ok() {
        commands.entity(trigger.target()).remove::<T>();
    }
}
