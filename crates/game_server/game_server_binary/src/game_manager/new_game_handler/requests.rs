use std::error::Error;

use arts_core::{
    auth_server::game::RequestNewGameRequest, game_meta::NewGameSettings, network::HttpRequestMeta,
};
use bevy_eventwork::async_trait;
use serde::{Deserialize, Serialize};
use tide::{http::Url, Endpoint, Request};

/// A request to start a new game
pub struct RequestNewGame {
    pub(crate) authentication_server_addr: Url,
}

#[async_trait]
impl Endpoint<()> for RequestNewGame {
    async fn call(&self, req: Request<()>) -> tide::Result {
        request_new_game(req, self.authentication_server_addr.clone()).await
    }
}

/// Handles requests to start a new game
async fn request_new_game(mut req: Request<()>, auth_server_addr: Url) -> tide::Result {
    let request: HttpRequestMeta<NewGameSettings> = req.body_json().await?;

    let mut request = ehttp::Request::post(
        format!("{}/game_management/request_new_game", auth_server_addr),
        serde_json::to_string(&HttpRequestMeta {
            request: RequestNewGameRequest {
                server_id: todo!(),
                game_ip: todo!(),
            },
        })
        .unwrap()
        .as_bytes()
        .to_vec(),
    );

    request
        .headers
        .insert("Content-Type".to_string(), "application/json".to_string());

    request.headers.insert(
        "autherization".to_string(),
        format!("Bearer {}", client_info.access_token.clone()),
    );

    let response = ehttp::fetch_async(request).await?;

    Ok(tide::Response::builder(200)
        .body(serde_json::to_string(&NewGameResponse {}).unwrap())
        .build())
}

#[derive(Serialize, Deserialize)]
pub struct NewGameResponse {}
