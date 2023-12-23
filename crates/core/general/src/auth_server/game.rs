use bevy::utils::Uuid;
use serde::{Deserialize, Serialize};

use crate::{game_meta::GameId, network::GameAddrInfo};

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
    pub server_id: Uuid,
    pub game_addr: GameAddrInfo,
}

/// Ok response returned from [`RequestNewGameRequests`]
///
/// ### Target:
/// Game Server
///
/// ### Sender:
/// Authentication Server
#[derive(Serialize, Deserialize)]
pub struct RequestNewGameIdResponse {
    pub game_id: GameId,
}
