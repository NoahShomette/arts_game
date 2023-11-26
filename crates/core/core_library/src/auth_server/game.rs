use bevy::utils::Uuid;
use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, PartialOrd, Ord, Eq)]
pub struct GameId {
    pub id: Uuid,
}

// ------------ HTTP Requests

/// Request a new game
///
/// ### Target:
/// Authentication Server
///
/// ### Sender:
/// Games Server
#[derive(Serialize, Deserialize)]
pub struct RequestNewGameRequest {
    /// The user_id from authentication.
    pub server_id: String,
    pub game_ip: Url,
}

/// Ok response returned from [`RequestNewGameRequests`]
///
/// ### Target:
/// Game Server
///
/// ### Sender:
/// Authentication Server
#[derive(Serialize, Deserialize)]
pub struct RequestNewGameResponse {
    pub game_id: GameId,
}
