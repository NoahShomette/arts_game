pub mod authentication;
mod game_server_connection;
pub mod ui;

use arts_core::{authentication::client_authentication::AuthClient, TaskPoolRes};
use authentication::AuthenticationPlugin;
use bevy::{app::Plugin, tasks::TaskPoolBuilder};
use game_server_connection::GameServerPlugin;

pub struct ClientPlugin;

impl Plugin for ClientPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_resource(AuthClient::new());
        app.add_plugins((AuthenticationPlugin, GameServerPlugin));
        app.insert_resource(TaskPoolRes(TaskPoolBuilder::new().num_threads(2).build()));
    }
}
