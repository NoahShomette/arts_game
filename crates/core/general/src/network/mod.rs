use bevy::ecs::system::Resource;
use serde::{Deserialize, Serialize};
use url::Url;

pub mod game_http;
pub mod ws_game_server;

/// A wrapper that contains meta information that clients/game_server/Auth_server can sends to any of the Servers that run http servers
/// in order to correctly make and process requests
///
/// Specific requests require this wrapper to desereialize the request properly. Basically all authentication requred requests
#[derive(Serialize, Deserialize)]
pub struct HttpRequestMeta<T> {
    pub request: T,
}

/// A struct that holds what a games address information is. Both HTTP and Websocket
///
/// server_addr should be just the address. Eg, 127.0.0.1 not http://127.0.0.1
#[derive(Resource, Serialize, Deserialize, Clone)]
pub struct GameAddrInfo {
    pub server_addr: String,
    pub http_port: u16,
    pub ws_port: u16,
}

impl GameAddrInfo {
    pub fn http_url(&self) -> Url {
        Url::parse(&format!(
            "http://{}:{}",
            self.server_addr.clone(),
            &self.http_port.to_string()
        ))
        .expect("Invalid address given")
    }

    pub fn ws_url(&self) -> Url {
        Url::parse(&format!(
            "ws://{}:{}",
            self.server_addr.clone(),
            &self.http_port.to_string()
        ))
        .expect("Invalid address given")
    }
}
