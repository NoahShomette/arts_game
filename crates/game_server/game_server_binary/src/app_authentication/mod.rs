//! Is responsible for authenticating the server. Eventually the server will require an account that has the Server role. This
//! will allow the server to also make http requests to change data for the data it owns. Eg delete a game, change a game ip, etc.

use bevy::app::Plugin;
use core_library::network::HttpRequestMeta;
use tide::{http::Url, Error};

pub struct AuthenticationPlugin;

impl Plugin for AuthenticationPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugins(core_library::authentication::CoreAuthenticationPlugin);
    }
}

/// Simple request that returns Ok if the access token is valid
pub async fn auth_user_request(access_token: String, auth_server_addr: Url) -> tide::Result<()> {
    let message = match serde_json::to_string(&HttpRequestMeta { request: () }) {
        Ok(message) => message.as_bytes().to_vec(),
        Err(err) => {
            return Err(Error::from_str(500, err));
        }
    };
    let mut request = ehttp::Request::post(
        format!("{}/game_management/request_new_game", auth_server_addr),
        message,
    );

    request
        .headers
        .insert("Content-Type".to_string(), "application/json".to_string());

    request.headers.insert(
        "autherization".to_string(),
        format!("Bearer {}", access_token),
    );

    match ehttp::fetch_async(request).await {
        Ok(_) => {
            return Ok(());
        }
        Err(err) => {
            return Err(Error::from_str(500, err));
        }
    };
}

/// Simple request that returns Ok if the access token is valid
pub fn auth_user_request_blocking(access_token: String, auth_server_addr: Url) -> tide::Result<()> {
    let message = match serde_json::to_string(&HttpRequestMeta { request: () }) {
        Ok(message) => message.as_bytes().to_vec(),
        Err(err) => {
            return Err(Error::from_str(500, err));
        }
    };
    let mut request = ehttp::Request::post(
        format!("{}/game_management/request_new_game", auth_server_addr),
        message,
    );

    request
        .headers
        .insert("Content-Type".to_string(), "application/json".to_string());

    request.headers.insert(
        "autherization".to_string(),
        format!("Bearer {}", access_token),
    );

    match ehttp::fetch_blocking(&request) {
        Ok(_) => {
            return Ok(());
        }
        Err(err) => {
            return Err(Error::from_str(500, err));
        }
    };
}
