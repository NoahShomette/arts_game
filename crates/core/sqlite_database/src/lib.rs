use async_mutex::Mutex;
use database_traits::DatabaseData;
use general::clone_async_sender;
use rusqlite::{params_from_iter, Error, Transaction};
use schemes::{
    game_server::{setup_game_server_schemes, GameServerPlugin},
    DatabaseSchemeAppExtension,
};
use update_row::UpdateRow;

use std::sync::Arc;

use bevy::{
    app::Plugin,
    ecs::{system::Resource, world::World},
    utils::HashMap,
};
use rusqlite::Connection;

pub mod database_traits;
pub mod schemes;
pub mod update_row;

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

        app.add_plugins(GameServerPlugin);

        app.server_register_sql_action::<UpdateRow>();
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

///
pub struct DatabaseDataRegistry {
    pub map: HashMap<&'static str, Box<dyn DatabaseData>>,
}

/// Fn that sets up the game world with everything needed by the save and database systems
pub fn game_world_setup_db(server_world: &World, game_world: &mut World) {
    game_world.insert_resource(
        clone_async_sender::<UpdateRow>(server_world)
            .expect("AsyncChannelSender<UpdateRow> not found"),
    );

    setup_game_server_schemes(server_world, game_world)
}

/// Tries to deserialize a possible bool from an i32 taken from the database
pub fn deserialize_bool(maybe_bool: i32) -> Option<bool> {
    match maybe_bool {
        0 => Some(false),
        1 => Some(true),
        _ => None,
    }
}
