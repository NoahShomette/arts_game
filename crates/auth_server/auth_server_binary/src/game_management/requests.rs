use async_trait::async_trait;
use bevy::utils::Uuid;
use core_library::{
    auth_server::game::{RequestNewGameIdResponse, RequestNewGameRequest},
    game_meta::GameId,
    network::HttpRequestMeta,
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
    if let Ok(mut connection) = database.connection.lock() {
        {
            // Simple verification that the requested server to host the game actually exists
            let mut stmt = connection.prepare(&format!(
                "SELECT server_id, server_type FROM server_data where server_id = {}",
                request.request.server_id
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
            &[
                &game_id.to_string(),
                &game_addr,
                "1",
                "1",
                &request.request.server_id.to_string(),
                &server_type.to_string(),
            ],
        );
        tx.commit()?;
    }
    Ok(tide::Response::builder(200)
        .body(
            serde_json::to_string(&RequestNewGameIdResponse {
                game_id: GameId { id: game_id },
            })
            .unwrap(),
        )
        .build())
}
