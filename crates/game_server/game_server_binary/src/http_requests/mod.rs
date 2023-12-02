//! Responsible for communicating with the Auth Server to request new game ids, update the database when meta information changes for a game, game ends, etc

use core_library::network::HttpRequestMeta;
use tide::{http::Url, Error};

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
