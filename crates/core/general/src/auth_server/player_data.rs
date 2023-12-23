use serde::{Deserialize, Serialize};
use url::Url;

use crate::game_meta::GameId;

#[derive(Serialize, Deserialize)]
pub struct PlayerGames {
    pub current_games: Vec<GameId>,
}

// ------------ HTTP Requests
/*
/// Request all of a players games
///
/// ### Target:
/// Authentication Server
///
/// ### Sender:
/// Client
#[derive(Serialize, Deserialize)]
pub struct PlayerGamesRequest {
    /// The user_id from authentication.
    pub server_id: g,
}
*/

/// Ok response returned from requests for all of a players games
///
/// ### Target:
/// Client
///
/// ### Sender:
/// Authentication Server
#[derive(Serialize, Deserialize)]
pub struct PlayerGamesResponse {
    /// Vec of GameId, Server URL, Server Type
    pub player_games: Vec<(GameId, Url, i32)>,
}
