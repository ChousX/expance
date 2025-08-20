use bevy::prelude::*;

pub struct AppPlugin;
impl Plugin for AppPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<AppState>();
        app.configure_sets(Update, (AppUpdate::PreData, AppUpdate::Data, AppUpdate::Action, AppUpdate::PostAction).chain());
    }
}

#[derive(States, Default, Clone, Eq, PartialEq, Debug, Hash)]
pub enum AppState {
    _First,
    _Splash,
    _Menu,
    _GameMenu,
    #[default]
    Game,
}

#[derive( SystemSet, Debug, Clone, Copy, Eq, PartialEq, Hash, PartialOrd, Ord)]
pub enum AppUpdate {
    PreData,
    Data,
    Action,
    PostAction,
}


