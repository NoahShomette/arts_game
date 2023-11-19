use serde::{Deserialize, Serialize};
use tide::http::Url;

use crate::auth_server::game::GameId;

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

/// Ok response returned from [`PlayerGamesRequest`]
///
/// ### Target:
/// Client
///
/// ### Sender:
/// Authentication Server
#[derive(Serialize, Deserialize)]
pub struct PlayerGamesResponse {
    pub player_games: Vec<(GameId, Url, i32)>,
}
