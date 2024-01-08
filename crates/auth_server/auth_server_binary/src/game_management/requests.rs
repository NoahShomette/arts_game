use async_trait::async_trait;
use bevy::utils::Uuid;
use core_library::{
    auth_server::{
        game::{
            GameAuthServerInfo, RequestNewGameIdResponse, RequestNewGameRequest,
            RequestOpenGamesResponse,
        },
        AccountId,
    },
    game_meta::GameId,
    network::{GameAddrInfo, HttpRequestMeta, ServerType},
};
use tide::{Endpoint, Error, Request};

use core_library::sqlite_database::Database;

/// A request to register a new game and return that result to the game server.
///
/// The only data that the auth server stores is the game id and the game servers ip.
pub struct RequestNewGame {
    pub(crate) database: Database,
}

#[async_trait]
impl Endpoint<()> for RequestNewGame {
    async fn call(&self, req: Request<()>) -> tide::Result {
        request_new_game(req, &self.database).await
    }
}

struct QueryResultRNG {
    _server_id: String,
    _server_type: i32,
}

/// Requests adding a new game into the auth database.
async fn request_new_game(mut req: Request<()>, database: &Database) -> tide::Result {
    let request: HttpRequestMeta<RequestNewGameRequest> = req.body_json().await?;
    let mut server_type = 3;
    let game_id = Uuid::new_v4();
    let mut connection = database.connection.lock().await;
    let account_id = serde_json::to_string(&AccountId {
        id: request.request.server_id,
    })?;
    {
        println!(
            "SELECT server_id, server_type FROM server_data where server_id = \'{}\'",
            account_id
        );

        // Simple verification that the requested server to host the game actually exists
        let mut stmt = connection.prepare(&format!(
            "SELECT server_id, server_type FROM server_data where \'{}\'",
            account_id
        ))?;
        let server = stmt.query_map((), |row| {
            Ok(QueryResultRNG {
                _server_id: row.get(0)?,
                _server_type: row.get(1)?,
            })
        })?;

        for server in server {
            let server_info = server?;
            server_type = server_info._server_type
        }
    }
    let tx = connection.transaction()?;
    let game_addr = match serde_json::to_string(&request.request.game_addr.clone()) {
        Ok(info) => info,
        Err(err) => return Err(Error::from_str(500, err)),
    };
    let _ = tx.execute(
            "insert into game_info (game_id, game_ip, is_open, in_progress, hosting_server_id, server_type) values (?1, ?2, ?3, ?4, ?5, ?6)",
            [
                &game_id.to_string(),
                &game_addr,
                "1",
                "1",
                &account_id,
                &server_type.to_string(),
            ],
        );
    tx.commit()?;

    let response = match serde_json::to_string(&RequestNewGameIdResponse {
        game_id: GameId { id: game_id },
    }) {
        Ok(body) => body.as_bytes().to_vec(),
        Err(err) => return Err(Error::from_str(500, err)),
    };

    Ok(tide::Response::builder(200).body(response).build())
}

/// A request to register a new game and return that result to the game server.
///
/// The only data that the auth server stores is the game id and the game servers ip.
pub struct RequestOpenGames {
    pub(crate) database: Database,
}

#[async_trait]
impl Endpoint<()> for RequestOpenGames {
    async fn call(&self, req: Request<()>) -> tide::Result {
        request_open_games(req, &self.database).await
    }
}

struct QueryResult {
    game_id: String,
    game_ip: String,
    is_open: i32,
    in_progress: i32,
    hosting_server_id: String,
    server_type: i32,
}

/// Requests a subset of open games in the auth server
async fn request_open_games(req: Request<()>, database: &Database) -> tide::Result {
    let Ok(offset) = req.param("offset")?.parse::<u32>() else {
        return Err(Error::from_str(500, "Failed to get offset from request"));
    };
    let connection = database.connection.lock().await;

    let mut open_games: Vec<GameAuthServerInfo> = vec![];
    {
        // Simple verification that the requested server to host the game actually exists
        let mut stmt = connection.prepare(&format!("SELECT game_id, game_ip, is_open, in_progress, hosting_server_id, server_type FROM server_data ORDER BY game_id LIMIT \'{}\', 15", offset))?;
        let games = stmt.query_map((), |row| {
            Ok(QueryResult {
                game_id: row.get("game_id")?,
                game_ip: row.get("game_ip")?,
                is_open: row.get("is_open")?,
                in_progress: row.get("in_progress")?,
                hosting_server_id: row.get("hosting_server_id")?,
                server_type: row.get("server_type")?,
            })
        })?;

        for game in games {
            let game_info = game?;
            let game_id = match serde_json::from_str::<GameId>(&game_info.game_id) {
                Ok(info) => info,
                Err(_) => continue,
            };
            let game_ip = match serde_json::from_str::<GameAddrInfo>(&game_info.game_ip) {
                Ok(info) => info,
                Err(_) => continue,
            };
            let is_open = matches!(game_info.is_open, 1);
            let in_progress = matches!(game_info.in_progress, 1);
            let Some(server_type) = ServerType::try_from_i32(game_info.server_type) else {
                continue;
            };
            let hosting_server_id =
                match serde_json::from_str::<AccountId>(&game_info.hosting_server_id) {
                    Ok(info) => info,
                    Err(_) => continue,
                };
            open_games.push(GameAuthServerInfo {
                game_id,
                game_ip,
                is_open,
                in_progress,
                hosting_server_id,
                server_type,
            });
        }
    }

    let response = match serde_json::to_string(&RequestOpenGamesResponse { games: open_games }) {
        Ok(body) => body.as_bytes().to_vec(),
        Err(err) => return Err(Error::from_str(500, err)),
    };

    Ok(tide::Response::builder(200).body(response).build())
}
