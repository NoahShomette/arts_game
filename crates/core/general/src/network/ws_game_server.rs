use bevy_eventwork::NetworkMessage;
use serde::{Deserialize, Serialize};

use crate::{auth_server::AccountId, game_meta::GameId};

/// Client message sent from the client to the game server to connect to a specific game
///
/// If the player is already connected to a game it disconnects them from that
#[derive(Serialize, Deserialize, Clone)]
pub struct ClientConnectToGame {
    pub game_id: GameId,
    pub access_token: String,
    pub player_id: AccountId,
}

impl NetworkMessage for ClientConnectToGame {
    const NAME: &'static str = "ClientConnectToGame";
}

/// Client message sent from the client to the game server to tell the game server what the client is
///
/// Must be sent as the first message from the client to the game server when connecting
#[derive(Serialize, Deserialize, Clone)]
pub struct ClientInitialConnect {
    pub access_token: String,
    pub player_id: AccountId,
}

impl NetworkMessage for ClientInitialConnect {
    const NAME: &'static str = "ClientInitialMessage";
}
