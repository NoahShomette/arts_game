use serde::{Deserialize, Serialize};

use crate::{auth_server::AccountId, game_meta::GameId};

#[derive(Serialize, Deserialize, Clone)]
pub struct JoinGame {
    pub game_id: GameId,
    pub player_id: AccountId,
}
