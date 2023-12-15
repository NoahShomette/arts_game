use bevy::app::Plugin;
use general::game_meta::{GameId, GamePlayers};

use crate::database_traits::DatabaseData;

use self::{game_tables::{InsertGameCurvesRow, CreateGamePlayersTable, CreateGameCurvesTable}, games_meta::InsertGamesMetaRow};

use super::DatabaseSchemeAppExtension;

pub mod game_actions;
pub mod game_tables;
pub mod games_meta;

pub(crate) struct GameServerPlugin;

impl Plugin for GameServerPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.register_sql_action::<InsertGamesMetaRow>();
        app.register_sql_action::<InsertGameCurvesRow>();
        app.register_sql_action::<CreateGameCurvesTable>();
        app.register_sql_action::<CreateGamePlayersTable>();
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
