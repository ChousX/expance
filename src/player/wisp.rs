use crate::app::AppUpdate;
use crate::cursor::CurrsorPositon;
use crate::helper::move_entity_to::{MoveEntityTo, Speed};
use crate::player::OwnedBy;
use crate::player::core::PlayerCore;
use bevy::prelude::*;
use bevy_asset_loader::asset_collection::AssetCollection;

pub struct PlayerWispPlugin;
impl Plugin for PlayerWispPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(spawn_player_wisp);
        app.add_systems(
            Update,
            (update_move_to, add_move_to)
                .chain()
                .in_set(AppUpdate::Data),
        );
        app.add_observer(on_halt::<HomeToCursor>)
            .add_observer(on_halt::<Halt>)
            .add_observer(on_halt::<MoveEntityTo>);
    }
}

#[derive(Component, Default)]
#[require(Transform)]
pub struct PlayerWisp;

#[derive(AssetCollection, Resource)]
pub struct PlayerWispSprite {
    #[asset(path = "placeholder/Diamond/Sprite-0002.png")]
    pub default: Handle<Image>,
}

#[derive(Component, Default)]
pub struct HomeToCursor;

fn spawn_player_wisp(
    trigger: Trigger<OnAdd, PlayerCore>,
    mut commands: Commands,
    transforms: Query<(&Transform, &OwnedBy)>,
    sprite_texture: Res<PlayerWispSprite>,
) {
    let (&transform, &OwnedBy(owner)) = transforms.get(trigger.target()).unwrap();
    commands.spawn((
        PlayerWisp,
        Sprite::from_image(sprite_texture.default.clone()),
        transform,
        Speed(5000.0),
        OwnedBy(owner),
        HomeToCursor,
    ));
    info!("spawned player wisp");
}

fn add_move_to(
    mut commands: Commands,
    cursor_pos: Res<CurrsorPositon>,
    wisp_q: Query<(&GlobalTransform, Entity), (With<PlayerWisp>, With<HomeToCursor>)>,
) {
    for (wisp_transform, entity) in wisp_q.iter() {
        let wisp_pos = wisp_transform.translation().xy();
        if wisp_pos != **cursor_pos {
            commands.entity(entity).insert(MoveEntityTo {
                to: **cursor_pos,
                from: wisp_pos,
                easing: EaseFunction::SmootherStepIn,
            });
        }
    }
}

fn update_move_to(
    mut wisp_q: Query<&mut MoveEntityTo, (With<PlayerWisp>, With<HomeToCursor>)>,
    cursor_pos: Res<CurrsorPositon>,
) {
    for mut move_to in wisp_q.iter_mut() {
        if move_to.to != **cursor_pos {
            move_to.to = **cursor_pos;
        }
    }
}

#[derive(Component, Default)]
pub struct Halt;

fn on_halt<T>(trigger: Trigger<OnAdd, Halt>, mut commands: Commands, query: Query<&T>)
where
    T: Component,
{
    if query.get(trigger.target()).is_ok() {
        commands.entity(trigger.target()).remove::<T>();
    }
}
