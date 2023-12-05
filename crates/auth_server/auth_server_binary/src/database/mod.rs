//! Responsible for saving games and keeping them backed up into the database

use std::sync::{Arc, Mutex};

use bevy::{app::Plugin, ecs::system::Resource};
use rusqlite::Connection;

pub struct DatabaseManagerPlugin;

impl Plugin for DatabaseManagerPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_resource(Database {
            connection: Arc::new(Mutex::new(
                Connection::open("databases/auth_database.db").expect("No Database found"),
            )),
        });
    }
}

/// Stores the active connection to the database used to save games
///
/// Can be freely cloned as its wrapped in Arc Mutex
#[derive(Resource, Clone)]
pub struct Database {
    pub connection: Arc<Mutex<rusqlite::Connection>>,
}
