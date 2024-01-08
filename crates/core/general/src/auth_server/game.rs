use bevy::utils::Uuid;
use serde::{Deserialize, Serialize};

use crate::{
    game_meta::GameId,
    network::{GameAddrInfo, ServerType},
};

use super::AccountId;

/// Info on each game that the auth server maintains
#[derive(Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct GameAuthServerInfo {
    pub game_id: GameId,
    pub game_ip: GameAddrInfo,
    pub is_open: bool,
    pub in_progress: bool,
    pub hosting_server_id: AccountId,
    pub server_type: ServerType,
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
    pub server_id: Uuid,
    pub game_addr: GameAddrInfo,
}

/// Ok response returned from [`RequestNewGameRequest`]
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

/// Ok response returned from requests for open games
///
/// ### Target:
/// Game Server
///
/// ### Sender:
/// Authentication Server
#[derive(Serialize, Deserialize)]
pub struct RequestOpenGamesResponse {
    pub games: Vec<GameAuthServerInfo>,
}
