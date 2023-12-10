//! Responsible for saving games and keeping them backed up into the database

use bevy::app::Plugin;
use core_library::sqlite_database::DatabasePlugin;

pub struct DatabaseManagerPlugin;

impl Plugin for DatabaseManagerPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugins(DatabasePlugin {
            database_path: "databases/auth_database.db".to_string(),
        });
    }
}
