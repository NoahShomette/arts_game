//! Responsible for saving games and keeping them backed up into the database

use std::sync::{Arc, Mutex};

use bevy::{app::Plugin, ecs::system::Resource};
use rusqlite::Connection;

pub struct SaveManagerPlugin;

impl Plugin for SaveManagerPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_resource(DatabaseConnection {
            connection: Arc::new(Mutex::new(
                Connection::open("databases/game_server_database.db").expect("No Database found"),
            )),
        });
    }
}

/// Stores the active connection to the database used to save games
#[derive(Resource)]
struct DatabaseConnection {
    connection: Arc<Mutex<rusqlite::Connection>>,
}
