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

use crate::database_traits::{DatabaseData, DatabaseTable};

use self::games::InsertGamesTableRow;

pub use super::DatabaseSchemeAppExtension;

pub mod games;

pub(crate) struct GameServerPlugin;

impl Plugin for GameServerPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_resource(GamesTable);

        app.server_register_sql_action::<InsertGamesTableRow>();
    }
}

/// Adds all the game server schemes to the server world
pub(crate) fn setup_game_server_schemes(server_world: &World, game_world: &mut World) {
    game_world.insert_resource(
        clone_async_sender::<InsertGamesTableRow>(server_world)
            .expect("AsyncChannelSender<InsertGamesMetaRow> not found"),
    );
    game_world.insert_resource(GamesTable);
}

#[derive(Resource)]
pub struct GamesTable;

impl DatabaseTable for GamesTable {
    fn table_name(&self) -> String {
        "games_meta".to_string()
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
