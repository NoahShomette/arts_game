use serde::{Deserialize, Serialize};

use crate::game::GameId;

#[derive(Serialize, Deserialize)]
pub struct PlayerGames {
    pub current_games: Vec<GameId>,
}
