use bevy::prelude::*;

use crate::app::AppState;

pub struct GamePlugin;
impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_sub_state::<PlayState>();
    }
}

#[derive(SubStates, Default, Clone, Eq, PartialEq, Debug, Hash)]
#[source(AppState = AppState::Game)]
pub enum PlayState {
    Paused,
    #[default]
    Playing,
}
