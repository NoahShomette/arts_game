use rusqlite::{params_from_iter, Error, Transaction};

use std::sync::{Arc, Mutex};

use bevy::{app::Plugin, ecs::system::Resource};
use rusqlite::Connection;

pub mod schemes;

pub struct DatabasePlugin {
    pub database_path: String,
}

impl Plugin for DatabasePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_resource(Database {
            connection: Arc::new(Mutex::new(
                Connection::open(&self.database_path).expect("No Database found"),
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

/// A trait used to enforce a single schema on the database
pub trait ConnectionSchema {
    /// Executes the desired schema. param.0 is the SQL command, param.1 are the params for the command
    fn execute_schema(&self, params: (String, Vec<String>)) -> Result<usize, Error>;
}

impl ConnectionSchema for Transaction<'_> {
    fn execute_schema(&self, params: (String, Vec<String>)) -> Result<usize, Error> {
        self.execute(&params.0, params_from_iter(params.1.iter()))
    }
}
