// Support configuring Bevy lints within code.
#![cfg_attr(bevy_lint, feature(register_tool), register_tool(bevy))]
// Disable console on Windows for non-dev builds.
#![cfg_attr(not(feature = "dev"), windows_subsystem = "windows")]

use bevy::prelude::*;

mod app;
mod camera;
mod chunk;
mod cursor;
mod domain;
mod game;
mod helper;
mod input;
mod player;
mod terrain;

fn main() -> AppExit {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins);
    app.add_plugins((
        app::AppPlugin,
        cursor::CurrsorPlugin,
        player::PlayerPlugin,
        camera::CameraPlugin,
        helper::HelperPlugin,
        terrain::TerrainPlugin,
        chunk::ChunkPlugin,
        game::GamePlugin,
        input::InputPlugin,
    ));
    app.run()
}
