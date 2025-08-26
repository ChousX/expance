use bevy::prelude::*;

use crate::app::AppState;

mod command;
mod core;
mod wisp;

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            core::PlayerCorePlugin,
            wisp::PlayerWispPlugin,
        ));
        app.add_systems(OnEnter(AppState::Game), spawn_player);
    }
}

#[derive(Component, Default)]
pub struct Player;

#[derive(Component, Default, Deref, DerefMut)]
pub struct PlayerId(pub u8);

fn spawn_player(mut commands: Commands) {
    commands.spawn(Player);
}

#[derive(Component)]
#[relationship_target(relationship = OwnedBy, linked_spawn)]
pub struct Owned(Vec<Entity>);

#[derive(Component)]
#[relationship(relationship_target = Owned)]
pub struct OwnedBy(pub Entity);
