use bevy::{
    app::Plugin,
    ecs::{system::Resource, world::World},
};
use bevy_state_curves::prelude::SteppedCurve;
use general::{
    clone_async_sender,
    game_meta::{GameId, GamePlayers},
    objects::core_components::{ObjectGeneral, ObjectId, ObjectPosition},
};

use crate::database_traits::{DatabaseData, DatabaseTable, GameDatabaseTable};

use self::{
    game_tables::{CreateGameCurvesTable, CreateGamePlayersTable, InsertGameCurvesRow},
    games_meta::InsertGamesMetaRow,
};

pub use super::DatabaseSchemeAppExtension;

pub mod game_actions;
pub mod game_tables;
pub mod games_meta;

pub(crate) struct GameServerPlugin;

impl Plugin for GameServerPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_resource(GamesMetaTable);
        app.insert_resource(GameCurvesTable);
        app.insert_resource(GamesPlayersTable);

        app.server_register_sql_action::<InsertGamesMetaRow>();
        app.server_register_sql_action::<InsertGameCurvesRow>();
        app.server_register_sql_action::<CreateGameCurvesTable>();
        app.server_register_sql_action::<CreateGamePlayersTable>();
    }
}

/// Adds all the game server schemes to the server world
pub(crate) fn setup_game_server_schemes(server_world: &World, game_world: &mut World) {
    game_world.insert_resource(
        clone_async_sender::<InsertGamesMetaRow>(server_world)
            .expect("AsyncChannelSender<InsertGamesMetaRow> not found"),
    );
    game_world.insert_resource(
        clone_async_sender::<InsertGameCurvesRow>(server_world)
            .expect("AsyncChannelSender<InsertGameCurvesRow> not found"),
    );
    game_world.insert_resource(
        clone_async_sender::<CreateGameCurvesTable>(server_world)
            .expect("AsyncChannelSender<CreateGameCurvesTable> not found"),
    );
    game_world.insert_resource(
        clone_async_sender::<CreateGamePlayersTable>(server_world)
            .expect("AsyncChannelSender<CreateGamePlayersTable> not found"),
    );
    game_world.insert_resource(GamesMetaTable);
    game_world.insert_resource(GameCurvesTable);
    game_world.insert_resource(GamesPlayersTable);
}

#[derive(Resource)]
pub struct GamesMetaTable;

impl DatabaseTable for GamesMetaTable {
    fn table_name(&self) -> String {
        "games_meta".to_string()
    }
}

#[derive(Resource)]
pub struct GameCurvesTable;

impl GameDatabaseTable for GameCurvesTable {
    fn table_name(&self, game_id: &GameId) -> String {
        format!("game_curves_{}", game_id.id_as_string())
    }
}

#[derive(Resource)]
pub struct GamesPlayersTable;

impl GameDatabaseTable for GamesPlayersTable {
    fn table_name(&self, game_id: &GameId) -> String {
        format!("game_players_{}", game_id.id_as_string())
    }
}

impl DatabaseData for GameId {
    fn to_database_string(&self) -> Option<String> {
        serde_json::to_string(self).ok()
    }

    fn column_name(&self) -> &str {
        "game_id"
    }
}

impl DatabaseData for GamePlayers {
    fn to_database_string(&self) -> Option<String> {
        serde_json::to_string(self).ok()
    }

    fn column_name(&self) -> &str {
        "game_players"
    }
}

impl DatabaseData for ObjectId {
    fn to_database_string(&self) -> Option<String> {
        serde_json::to_string(self).ok()
    }

    fn column_name(&self) -> &str {
        "object_id"
    }
}

impl DatabaseData for ObjectGeneral {
    fn to_database_string(&self) -> Option<String> {
        serde_json::to_string(self).ok()
    }

    fn column_name(&self) -> &str {
        "object_general"
    }
}

impl DatabaseData for SteppedCurve<ObjectPosition> {
    fn to_database_string(&self) -> Option<String> {
        serde_json::to_string(self).ok()
    }

    fn column_name(&self) -> &str {
        "sc_object_position"
    }
}
