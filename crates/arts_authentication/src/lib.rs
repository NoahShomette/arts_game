use authentication::AuthenticationPlugin;
use bevy::app::Plugin;
use database::DatabaseManagerPlugin;

pub mod authentication;
pub mod database;
pub mod game_management;

pub struct ServerLibraryPlugin;

impl Plugin for ServerLibraryPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugins((DatabaseManagerPlugin, AuthenticationPlugin));
    }
}
