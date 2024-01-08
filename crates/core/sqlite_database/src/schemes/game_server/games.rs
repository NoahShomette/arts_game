use bevy::ecs::component::Component;
use general::{
    auth_server::AccountId,
    game_meta::{GameId, GamePlayers},
    game_state::{GameActions, GameStatePatches, ObjectsState},
    objects::ObjectIdService,
};

use crate::database_traits::DatabaseSql;

#[derive(Component, Debug, Clone)]
pub struct InsertGamesTableRow {
    pub game_id: GameId,
    pub max_players: u8,
    pub owning_player: Option<AccountId>,
    pub object_id_service: ObjectIdService,
    pub game_name: String,
    pub starting_state: ObjectsState,
    pub game_actions: GameActions,
    pub game_state_patches: GameStatePatches,
}

impl DatabaseSql for InsertGamesTableRow {
    fn to_sql(&self) -> Option<(String, Vec<String>)> {
        let game_id = self.game_id.to_json();

        let Ok(object_id_service) = serde_json::to_string(&self.object_id_service) else {
            return None;
        };

        let Ok(starting_state) = serde_json::to_string(&self.starting_state) else {
            return None;
        };

        let Ok(game_actions) = serde_json::to_string(&self.game_actions) else {
            return None;
        };

        let Ok(game_state_patches) = serde_json::to_string(&self.game_state_patches) else {
            return None;
        };

        match &self.owning_player {
            Some(player) => {
                let Ok(game_players) =
                    serde_json::to_string(&GamePlayers::new_with_id(player.clone()))
                else {
                    return None;
                };
                let Ok(owning_player) = serde_json::to_string(&player) else {
                    return None;
                };

                Some(("insert into games (game_id, game_players, max_players, game_state, has_space, object_id_service, owning_player, game_name) values (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)".to_string(),
                vec![
                    game_id,
                    game_players,
                    self.max_players.to_string(),
                    0.to_string(),
                    1.to_string(),
                    object_id_service,
                    owning_player,
                    self.game_name.clone(),
                    starting_state,
                    game_actions,
                    game_state_patches
                ],
            ))
            }
            None => {
                let Ok(game_players) = serde_json::to_string(&GamePlayers::default()) else {
                    return None;
                };
                Some((
                "insert into games (game_id, game_players, max_players, game_state, has_space, object_id_service, game_name, starting_state, actions, patches) values (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)".to_string(),
                vec![
                    game_id,
                    game_players,
                    self.max_players.to_string(),
                    0.to_string(),
                    1.to_string(),
                    object_id_service,
                    self.game_name.clone(),
                    starting_state,
                    game_actions,
                    game_state_patches
                ],
            ))
            }
        }
    }
}
