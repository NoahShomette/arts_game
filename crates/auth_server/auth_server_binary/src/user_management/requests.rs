use std::sync::Arc;

use arts_core::auth_server::{
    game::GameId,
    player_data::{PlayerGames, PlayerGamesResponse},
};
use async_trait::async_trait;
use bevy::utils::Uuid;
use tide::{http::Url, Endpoint, Error, Request};

use crate::{authentication::supabase::SupabaseConnection, database::Database};

/// A request to register a new game and return that result to the game server.
///
/// The only data that the auth server stores is the game id and the game servers ip.
pub struct RequestPlayerGames {
    pub(crate) supabase: Arc<SupabaseConnection>,
    pub(crate) database: Database,
}

#[async_trait]
impl Endpoint<()> for RequestPlayerGames {
    async fn call(&self, req: Request<()>) -> tide::Result {
        request_player_games(req, &self.supabase, &self.database).await
    }
}

struct QueryResultRPG {
    player_games: String,
}

struct QueryResultGames {
    game_id: String,
    game_ip: String,
    server_type: i32,
}

/// Requests all of a players games and the neccessary data for them.
async fn request_player_games(
    req: Request<()>,
    supabase: &SupabaseConnection,
    database: &Database,
) -> tide::Result {
    let Some(access_token) = req.header("authorization") else {
        return Err(Error::from_str(400, "No Authorization Bearer found"));
    };

    let string_at = access_token.to_string();

    let access_token = string_at.split_whitespace().collect::<Vec<&str>>()[1];

    let Ok(claims) = supabase.jwt_valid(access_token) else {
        return Err(Error::from_str(403, "Invalid Acess Token"));
    };
    let mut games_mapped: Vec<(GameId, Url, i32)> = vec![];
    if let Ok(connection) = database.connection.lock() {
        {
            let mut stmt = connection.prepare(&format!(
                "SELECT player_games FROM player_data where player_id = {}",
                claims.sub
            ))?;

            let games = stmt.query_map((), |row| {
                Ok(QueryResultRPG {
                    player_games: row.get(1)?,
                })
            })?;

            for game in games {
                let game: PlayerGames = serde_json::from_str(&game?.player_games).unwrap();

                for game in game.current_games.iter() {
                    let mut stmt = connection.prepare(&format!(
                        "SELECT game_id, game_ip, server_type FROM game_data where game_id = {}",
                        game.id
                    ))?;

                    let games_data = stmt.query_map((), |row| {
                        Ok(QueryResultGames {
                            game_id: row.get(1)?,
                            game_ip: row.get(2)?,
                            server_type: row.get(6)?,
                        })
                    })?;

                    for game in games_data {
                        let game_info = game?;
                        games_mapped.push((
                            GameId {
                                id: Uuid::parse_str(&game_info.game_id)?,
                            },
                            Url::parse(&game_info.game_ip)?,
                            game_info.server_type,
                        ))
                    }
                }
            }
        }
    }
    Ok(tide::Response::builder(200)
        .body(
            serde_json::to_string(&PlayerGamesResponse {
                player_games: games_mapped,
            })
            .unwrap(),
        )
        .build())
}
