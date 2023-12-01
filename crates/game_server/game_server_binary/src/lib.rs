use app_authentication::AuthenticationPlugin;
use bevy::{app::Plugin, tasks::TaskPoolBuilder};
use core_library::TaskPoolRes;
use game_runner::GameRunnerPlugin;
use game_server_connection::GameServerPlugin;

mod app_authentication;
mod game_manager;
mod game_runner;
mod game_server_connection;
mod http_requests;
mod player_actions;

pub struct ServerPlugin;

impl Plugin for ServerPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_resource(TaskPoolRes(TaskPoolBuilder::new().num_threads(2).build()));

        // Game Server Plugin must be inserted first so that the GameWorldSimulationSchedule is available to all other plugins
        app.add_plugins(GameServerPlugin);
        app.add_plugins((AuthenticationPlugin, GameRunnerPlugin));
    }
}
