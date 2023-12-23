use app::GameAppPlugin;
use app_authentication::AuthenticationPlugin;
use bevy::{app::Plugin, tasks::TaskPoolBuilder};
use client_game_server_network::GameServerPlugin;
use core_library::TaskPoolRes;
use game_manager::GameManagerPlugin;
use game_runner::GameRunnerPlugin;
use http_network::HttpNetworkPlugin;

mod app;
mod app_authentication;
mod client_game_server_network;
mod game_manager;
mod game_meta;
mod game_runner;
mod http_network;
mod player_actions;

pub struct ServerPlugin;

impl Plugin for ServerPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_resource(TaskPoolRes(TaskPoolBuilder::new().build()));

        // Game Server Plugin must be inserted first so that the GameWorldSimulationSchedule is available to all other plugins
        app.add_plugins(GameServerPlugin);
        app.add_plugins((
            GameAppPlugin,
            AuthenticationPlugin,
            GameRunnerPlugin,
            GameManagerPlugin,
            HttpNetworkPlugin,
        ));
    }
}
