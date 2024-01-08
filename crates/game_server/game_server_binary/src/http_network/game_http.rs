//! General http requests related to games.

use bevy::ecs::system::{Res, ResMut};
use bevy_eventwork::async_trait;
use chrono::{DateTime, Utc};
use core_library::{
    authentication::AuthenticationServerInfo,
    game_meta::GameStateEnum,
    http_server::{request_access_token, TideServerResource},
    network::{
        game_http::{GameMetaInfo, GamesInfoResponse, RequestGamesInfo},
        HttpRequestMeta,
    },
    sqlite_database::{deserialize_bool, Database},
};
use tide::{http::Url, Endpoint, Error, Request};

use crate::app_authentication::auth_user_request;

pub fn add_game_info_request(
    mut tide: ResMut<TideServerResource>,
    auth: Res<AuthenticationServerInfo>,
    database: Res<Database>,
) {
    tide.0.at("/games/games_info").post(RequestGameInformation {
        authentication_server_addr: auth.addr.clone(),
        database: database.clone(),
    });
}

/// A request for meta information on games.
pub struct RequestGameInformation {
    authentication_server_addr: Url,
    pub(crate) database: Database,
}

#[async_trait]
impl Endpoint<()> for RequestGameInformation {
    async fn call(&self, req: Request<()>) -> tide::Result {
        request_game_information(req, &self.database, self.authentication_server_addr.clone()).await
    }
}

struct QueryResultRPG {
    game_id: String,
    game_players: String,
    max_players: i32,
    game_state: i32,
    is_open: i32,
    has_space: i32,
    owning_player: String,
    game_name: String,
    game_start_time: String,
}

/// Handles requests for information for meta information on games
async fn request_game_information(
    mut req: Request<()>,
    database: &Database,
    auth_server_addr: Url,
) -> tide::Result {
    let request: HttpRequestMeta<RequestGamesInfo> = req.body_json().await?;

    let Ok(request_access_token) = request_access_token(&req) else {
        return Err(Error::from_str(500, "No Access Token"));
    };
    auth_user_request(request_access_token, auth_server_addr.clone()).await?;

    let mut games_info: Vec<GameMetaInfo> = vec![];

    let connection = database.connection.lock().await;

    for game_id in request.request.games.iter() {
        let mut stmt = connection.prepare(&format!(
            "SELECT game_id, game_players, max_players, game_state, is_open, has_space, owning_player, game_name, game_start_time FROM games where game_id = \'{}\'",
            game_id.to_json()
        ))?;

        let games = stmt.query_map((), |row| {
            Ok(QueryResultRPG {
                game_id: row.get("game_ip")?,
                game_players: row.get("game_players")?,
                max_players: row.get("max_players")?,
                game_state: row.get("game_state")?,
                is_open: row.get("is_open")?,
                has_space: row.get("has_space")?,
                owning_player: row.get("owning_player")?,
                game_name: row.get("game_name")?,
                game_start_time: row.get("game_start_time")?,
            })
        })?;

        for game in games {
            let Ok(game) = game else {
                continue;
            };
            let Ok(game_players) = serde_json::from_str(&game.game_players) else {
                continue;
            };
            let Ok(game_id) = serde_json::from_str(&game.game_id) else {
                continue;
            };
            let owning_player = serde_json::from_str(&game.owning_player).ok();
            let Some(game_state) = GameStateEnum::try_from_i32(game.game_state) else {
                continue;
            };

            let Some(has_space) = deserialize_bool(game.has_space) else {
                continue;
            };
            let Some(is_open) = deserialize_bool(game.is_open) else {
                continue;
            };

            let game_start_time = serde_json::from_str::<DateTime<Utc>>(&game.game_start_time).ok();

            games_info.push(GameMetaInfo {
                game_id,
                game_players,
                max_players: game.max_players as u8,
                game_state,
                has_space,
                is_open,
                game_name: game.game_name,
                owning_player,
                game_start_time,
            });
        }
    }

    let response = match serde_json::to_string(&GamesInfoResponse { games: games_info }) {
        Ok(body) => body.as_bytes().to_vec(),
        Err(err) => return Err(Error::from_str(500, err)),
    };

    Ok(tide::Response::builder(200).body(response).build())
}
