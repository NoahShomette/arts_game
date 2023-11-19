use arts_core::{game::GameId, network::ClientHttpRequest};
use async_trait::async_trait;
use bevy::utils::Uuid;
use serde::{Deserialize, Serialize};
use tide::{http::Url, Endpoint, Request};

use crate::database::Database;

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

#[derive(Serialize, Deserialize)]
pub struct RequestNewGameRequest {
    /// The user_id from authentication.
    pub server_id: String,
    pub game_ip: Url,
}

#[derive(Serialize, Deserialize)]
pub struct RequestNewGameResponse {
    pub game_id: GameId,
}

struct QueryResult {
    _server_id: String,
    _server_type: i32,
}

/// Registers a new game into the auth database.
async fn request_new_game(mut req: Request<()>, database: &Database) -> tide::Result {
    let request: ClientHttpRequest<RequestNewGameRequest> = req.body_json().await?;
    let game_id = Uuid::new_v4();
    if let Ok(mut connection) = database.connection.lock() {
        {
            // Simple verification that the requested server to host the game actually exists
            let mut stmt = connection.prepare(&format!(
                "SELECT server_id, server_type FROM server_data where server_id = {}",
                request.request.server_id
            ))?;

            let server = stmt.query_map((), |row| {
                Ok(QueryResult {
                    _server_id: row.get(0)?,
                    _server_type: row.get(1)?,
                })
            })?;

            for server in server {
                let _ = server?;
            }
        }

        let tx = connection.transaction()?;
        let _ = tx.execute(
            "insert into game_info (game_id, game_ip, is_open, in_progress, hosting_server_id) values (?1, ?2, ?3, ?4, ?5)",
            &[
                &game_id.to_string(),
                &request.request.game_ip.to_string(),
                "1",
                "1",
                &request.request.server_id,
            ],
        );
        tx.commit()?;
    }
    Ok(tide::Response::builder(200)
        .body(
            serde_json::to_string(&RequestNewGameResponse {
                game_id: GameId { id: game_id },
            })
            .unwrap(),
        )
        .build())
}
