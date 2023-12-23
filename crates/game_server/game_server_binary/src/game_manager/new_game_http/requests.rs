use std::sync::{
    mpsc::{self, Receiver, Sender},
    Arc, Mutex,
};

use bevy::{ecs::system::Resource, utils::Uuid};
use bevy_eventwork::async_trait;
use core_library::{
    auth_server::{
        game::{RequestNewGameIdResponse, RequestNewGameRequest},
        AccountId,
    },
    authentication::client_authentication::Claims,
    game_meta::{GameId, NewGameSettings},
    http_server::request_access_token,
    network::{GameAddrInfo, HttpRequestMeta},
};
use serde::{Deserialize, Serialize};
use tide::{http::Url, Endpoint, Error, Request};

use crate::{app_authentication::auth_user_request, game_manager::new_game::NewGameCommand};

/// A request to start a new game
pub struct RequestNewGame {
    pub(crate) access_token: String,
    pub(crate) authentication_server_addr: Url,
    pub(crate) self_server_id: Uuid,
    pub(crate) game_ip: GameAddrInfo,
    pub(crate) channel: NewGameCommandsChannel,
}

#[async_trait]
impl Endpoint<()> for RequestNewGame {
    async fn call(&self, req: Request<()>) -> tide::Result {
        request_new_game(
            req,
            self.access_token.clone(),
            self.authentication_server_addr.clone(),
            self.game_ip.clone(),
            self.self_server_id.clone(),
            self.channel.clone(),
        )
        .await
    }
}

#[derive(Serialize, Deserialize)]
pub struct NewGameResponse {
    pub game_id: GameId,
    pub game_ip: GameAddrInfo,
}

/// Handles requests to start a new game
///
/// Verifies that the player is valid before it does so
async fn request_new_game(
    mut req: Request<()>,
    server_access_token: String,
    auth_server_addr: Url,
    game_ip: GameAddrInfo,
    self_server_id: Uuid,
    channel: NewGameCommandsChannel,
) -> tide::Result {
    let request: HttpRequestMeta<NewGameSettings> = req.body_json().await?;
    let access_token = request_access_token(&req)?;
    let body = auth_user_request(access_token.clone(), auth_server_addr.clone()).await?;
    let requesting_player: Option<AccountId> = match body.text() {
        Some(text) => {
            let claims: Claims = serde_json::from_str(text).unwrap();
            match Uuid::parse_str(&claims.sub) {
                Ok(id) => Some(AccountId { id: id }),
                Err(_) => None,
            }
        }
        None => None,
    };
    let new_game_id = request_new_game_id(
        server_access_token,
        auth_server_addr,
        game_ip.clone(),
        self_server_id,
    )
    .await?;

    let _ = channel.sender_channel.send(NewGameCommand {
        new_game_settings: request.request,
        new_game_id,
        owning_player: requesting_player,
    });
    Ok(tide::Response::builder(200)
        .body(
            serde_json::to_string(&NewGameResponse {
                game_id: new_game_id,
                game_ip,
            })
            .unwrap(),
        )
        .build())
}

#[derive(Resource, Clone)]
pub struct NewGameCommandsChannel {
    pub sender_channel: Sender<NewGameCommand>,
    pub reciever_channel: Arc<Mutex<Receiver<NewGameCommand>>>,
}

impl NewGameCommandsChannel {
    pub fn new() -> Self {
        let (sender, reciever) = mpsc::channel::<NewGameCommand>();

        NewGameCommandsChannel {
            sender_channel: sender,
            reciever_channel: Arc::new(Mutex::new(reciever)),
        }
    }
}

/// Request to Auth Server for a new Game Id
async fn request_new_game_id(
    access_token: String,
    auth_server_addr: Url,
    game_ip: GameAddrInfo,
    self_server_id: Uuid,
) -> tide::Result<GameId> {
    let message = match serde_json::to_string(&HttpRequestMeta {
        request: RequestNewGameRequest {
            server_id: self_server_id,
            game_addr: game_ip,
        },
    }) {
        Ok(message) => message.as_bytes().to_vec(),
        Err(err) => {
            return Err(Error::from_str(500, err));
        }
    };
    let mut request = ehttp::Request::post(
        format!("{}games/request_new_game", auth_server_addr),
        message,
    );

    request
        .headers
        .insert("Content-Type".to_string(), "application/json".to_string());

    request.headers.insert(
        "authorization".to_string(),
        format!("Bearer {}", access_token),
    );

    match ehttp::fetch_async(request).await {
        Ok(response) => match response.status {
            200 => {
                let new_game_id: RequestNewGameIdResponse =
                    serde_json::from_str(response.text().unwrap()).unwrap();
                return Ok(new_game_id.game_id);
            }
            _ => return Err(Error::from_str(response.status, response.status_text)),
        },
        Err(err) => {
            return Err(Error::from_str(500, err));
        }
    };
}
