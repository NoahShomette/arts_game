//! Responsible for saving games and keeping them backed up into the database

use bevy::app::Plugin;
use core_library::sqlite_database::DatabasePlugin;

pub struct GameDatabasePlugin;

impl Plugin for GameDatabasePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugins(DatabasePlugin {
            database_path: "databases/game_server_database.db".to_string(),
        });
    }
}
