use app_authentication::AuthenticationPlugin;
use bevy::app::Plugin;
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
        app.add_plugins((GameServerPlugin, AuthenticationPlugin, GameRunnerPlugin));
    }
}
