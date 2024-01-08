use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::{
    auth_server::AccountId,
    game_meta::{GameId, GamePlayers, GameStateEnum},
};

#[derive(Serialize, Deserialize, Clone)]
pub struct GameMetaInfo {
    pub game_id: GameId,
    pub game_players: GamePlayers,
    pub max_players: u8,
    pub game_state: GameStateEnum,
    pub is_open: bool,
    pub has_space: bool,
    pub owning_player: Option<AccountId>,
    pub game_name: String,
    pub game_start_time: Option<DateTime<Utc>>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct JoinGame {
    pub game_id: GameId,
    pub player_id: AccountId,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct QuitGame {
    pub game_id: GameId,
    pub player_id: AccountId,
}

/// A request for all the game info for the given [`GameId`]s
#[derive(Serialize, Deserialize, Clone)]
pub struct RequestGamesInfo {
    pub games: Vec<GameId>,
}

/// A response for [`RequestGamesInfo`]
///
/// The vec contains in order
/// ([`GameId`],
/// [`GamePlayers`],
/// max players,
/// the current game state,
/// bool representing if the game has space available,
/// the [`AcountId`] of the player that owns the game,
/// the name of the game)
#[derive(Serialize, Deserialize, Clone)]
pub struct GamesInfoResponse {
    pub games: Vec<GameMetaInfo>,
}
