use std::sync::Arc;

use crate::authentication::supabase::SupabaseConnection;
use crate::user_management::verify_decode_jwt;
use async_trait::async_trait;
use bevy::utils::Uuid;
use core_library::{
    auth_server::player_data::{PlayerGames, PlayerGamesResponse},
    game_meta::GameId,
    network::HttpRequestMeta,
    player::{SetPlayerUsernameRequest, SetPlayerUsernameResponse, MAX_USERNAME_LENGTH},
};
use rustrict::{CensorStr, Type};
use tide::{http::Url, Endpoint, Error, Request};

use core_library::sqlite_database::Database;

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
    let claims = verify_decode_jwt(&req, supabase)?;
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

/// A request to set a players username.
///
/// checks if its a valid username before assigning it
pub struct SetPlayerUsername {
    pub(crate) supabase: Arc<SupabaseConnection>,
    pub(crate) database: Database,
}

struct QueryResultPlayerUsernames {
    _username: String,
}

#[async_trait]
impl Endpoint<()> for SetPlayerUsername {
    async fn call(&self, req: Request<()>) -> tide::Result {
        set_player_username(req, &self.supabase, &self.database).await
    }
}

/// sets a player username to the given request
async fn set_player_username(
    mut req: Request<()>,
    supabase: &SupabaseConnection,
    database: &Database,
) -> tide::Result {
    // Verify its a real user
    let claims = verify_decode_jwt::<()>(&req, supabase)?;
    let request: HttpRequestMeta<SetPlayerUsernameRequest> = req.body_json().await?;
    if let Err(err) = is_valid_username(&request.request.username) {
        return Err(Error::from_str(
            500,
            format!("Username is invalid - Error: {}", err),
        ));
    }

    if let Ok(connection) = database.connection.lock() {
        {
            let mut stmt = connection.prepare(&format!(
                "SELECT username FROM player_data where username = \"{}\"",
                request.request.username
            ))?;

            let mut users = stmt.query_map((), |row| {
                Ok(QueryResultPlayerUsernames {
                    _username: row.get("username")?,
                })
            })?;

            if users.next().is_none() {
                let mut stmt = connection.prepare(&format!(
                    "UPDATE player_data SET username = \"{}\" where player_id = {}",
                    request.request.username, claims.sub
                ))?;
                stmt.execute([])?;
            } else {
                return Err(Error::from_str(500, "Username is already taken"));
            }
        }
    }
    Ok(tide::Response::builder(200)
        .body(
            serde_json::to_string(&SetPlayerUsernameResponse {
                new_username: request.request.username.clone(),
            })
            .unwrap(),
        )
        .build())
}

/// Returns true if the username is valid. Returns false if it is not allowed
fn is_valid_username(username: &str) -> Result<(), String> {
    if username.is(Type::MODERATE_OR_HIGHER) {
        return Err("Contains banned words".to_string());
    } else if username.chars().count() >= MAX_USERNAME_LENGTH {
        return Err(format!(
            "Username is longer than the allowed limit: {} characters",
            MAX_USERNAME_LENGTH
        ));
    } else {
        return Ok(());
    }
}

#[cfg(test)]
mod tests {
    use crate::user_management::requests::is_valid_username;

    #[test]
    fn test_username_filter() {
        // check_valid_username should return false here
        let username = "FUCK THIS";
        assert!(is_valid_username(username).is_err());

        // Should return true here
        let username = "FRANK THIS";
        assert!(is_valid_username(username).is_ok());
    }
}
