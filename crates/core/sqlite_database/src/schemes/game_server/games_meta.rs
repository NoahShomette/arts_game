use bevy::ecs::component::Component;
use general::{
    game_meta::{GameId, GamePlayers},
    objects::ObjectIdService,
};

use crate::database_traits::DatabaseSql;

#[derive(Component, Debug)]
pub struct InsertGamesMetaRow {
    pub game_id: GameId,
    pub max_players: u8,
    pub object_id_service: ObjectIdService,
}

impl DatabaseSql for InsertGamesMetaRow {
    fn to_sql(&self) -> Option<(String, Vec<String>)> {
        let game_id = self.game_id.to_json();

        Some((
            "insert into games_meta (game_id, game_players, max_players, game_state, has_space, pending_players, object_id_service) values (?1, ?2, ?3, ?4, ?5, ?6, ?7)".to_string(),
            vec![
                game_id,
                serde_json::to_string(&GamePlayers::new())
                .unwrap(),
                self.max_players.to_string(),
                0.to_string(),
                1.to_string(),
                0.to_string(),
                serde_json::to_string(&self.object_id_service)
                .unwrap()
            ],
        ))
    }
}
