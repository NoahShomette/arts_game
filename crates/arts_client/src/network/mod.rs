use bevy::app::Plugin;

use self::{authentication::AuthenticationPlugin, game_server_connection::GameServerPlugin};

pub mod authentication;
mod game_server_connection;

pub struct NetworkPlugin;

impl Plugin for NetworkPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugins((AuthenticationPlugin, GameServerPlugin));
    }
}
