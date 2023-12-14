use general::game_meta::{GameId, GamePlayers};

use crate::database_traits::DatabaseData;

pub mod game_actions;
pub mod game_tables;
pub mod games_meta;

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
