use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

pub struct AppPlugin;
impl Plugin for AppPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<AppState>()
            .add_sub_state::<AppLoadingState>();

        app.configure_sets(
            Update,
            (
                AppUpdate::PreData,
                AppUpdate::Data,
                AppUpdate::Action,
                AppUpdate::PostAction,
            )
                .chain(),
        );

        app.add_loading_state(
            LoadingState::new(AppLoadingState::Loading)
                .continue_to_state(AppLoadingState::Loaded)
                .load_collection::<crate::terrain::RawTileTextures>(),
        );
    }
}

#[derive(States, Default, Clone, Eq, PartialEq, Debug, Hash)]
pub enum AppState {
    #[default]
    First,
    Splash,
    _Menu,
    _GameMenu,
    Game,
}

#[derive(SubStates, Default, Clone, Eq, PartialEq, Debug, Hash)]
#[source(AppState = AppState::Splash)]
pub enum AppLoadingState {
    #[default]
    Loading,
    Loaded,
}

#[derive(SystemSet, Debug, Clone, Copy, Eq, PartialEq, Hash, PartialOrd, Ord)]
pub enum AppUpdate {
    PreData,
    Data,
    Action,
    PostAction,
}
