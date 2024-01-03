//! Responsible for handling players Joining and Leaving games that have not already started, eg are still in the pregame lobby.
//!
//! Joining and leaving in progress games will need to look a bit differently but this can probably easily be updated to do that in the future

use std::sync::mpsc::Sender;

use bevy::ecs::system::{Res, ResMut};
use bevy_eventwork::async_trait;
use core_library::{
    authentication::{client_authentication::ClientAuthenticationInfo, AuthenticationServerInfo},
    game_meta::GamePlayers,
    http_server::{request_access_token, TideServerResource},
    network::{
        game_http::{JoinGame, QuitGame},
        HttpRequestMeta,
    },
    sqlite_database::{
        database_traits::{DatabaseData, PureDatabaseData},
        update_row::UpdateRow,
        Database,
    },
    AsyncChannelSender,
};
use tide::{http::Url, Endpoint, Error, Request};

use crate::app_authentication::auth_user_request;

pub fn add_join_and_quit_request(
    mut tide: ResMut<TideServerResource>,
    auth: Res<AuthenticationServerInfo>,
    client: Res<ClientAuthenticationInfo>,
    database: Res<Database>,
    update_row_channel: Res<AsyncChannelSender<UpdateRow>>,
) {
    tide.0.at("/games/join_game").get(JoinGameEndpoint {
        authentication_server_addr: auth.addr.clone(),
        database: database.clone(),
        update_row_channel: update_row_channel.sender_channel.clone(),
    });
    tide.0.at("/games/quit_game").get(QuitGameEndpoint {
        authentication_server_addr: auth.addr.clone(),
        server_access_token: client.sign_in_info.access_token.clone(),
        database: database.clone(),
        update_row_channel: update_row_channel.sender_channel.clone(),
    });
}

/// A request to join a game
pub struct JoinGameEndpoint {
    pub(crate) authentication_server_addr: Url,
    pub(crate) database: Database,
    pub(crate) update_row_channel: Sender<UpdateRow>,
}

#[async_trait]
impl Endpoint<()> for JoinGameEndpoint {
    async fn call(&self, req: Request<()>) -> tide::Result {
        join_game(
            req,
            self.authentication_server_addr.clone(),
            self.database.clone(),
            self.update_row_channel.clone(),
        )
        .await
    }
}

struct DbQuery {
    game_players: String,
    max_players: u8,
    owning_player: Option<String>,
}

/// Handles requests to join a game
///
/// Verifies that the player is valid before it adds them to the game
async fn join_game(
    mut req: Request<()>,
    auth_server_addr: Url,
    database: Database,
    update_row_channel: Sender<UpdateRow>,
) -> tide::Result {
    let request: HttpRequestMeta<JoinGame> = req.body_json().await?;

    let Ok(request_access_token) = request_access_token(&req) else {
        return Err(Error::from_str(500, "No Access Token"));
    };
    auth_user_request(request_access_token, auth_server_addr.clone()).await?;

    // Simple verification that the player can join the game - an ok will be returned and the game server will add the player as soon as it can

    let mut connection = database.connection.lock().await;
    let tx = connection.transaction()?;
    {
        let mut stmt = tx.prepare(&format!(
                "SELECT game_players, max_players, pending_players, owning_player FROM games_meta where \"game_id = {}\" AND has_space = {} AND game_state = {}",
                request.request.game_id.to_json(),
                &1.to_string(),
                &0.to_string(),
            ))?;

        let server = stmt.query_map((), |row| {
            Ok(DbQuery {
                game_players: row.get(1)?,
                max_players: row.get(2)?,
                owning_player: row.get(7)?,
            })
        })?;

        let Some(game_id) = request.request.game_id.to_database_data() else {
            return Err(Error::from_str(500, "Invalid game_id"));
        };

        for server in server {
            let server_info = server?;
            let mut game_players =
                match serde_json::from_str::<GamePlayers>(&server_info.game_players) {
                    Ok(info) => info,
                    Err(err) => return Err(Error::from_str(500, err)),
                };

            if game_players.contains(&request.request.player_id)
                || game_players.count() >= server_info.max_players
            {
                return Err(Error::from_str(500, "Player not able to join game"));
            }

            game_players.insert(request.request.player_id.clone());
            let Ok(mut update_row) =
                UpdateRow::new("games_meta".to_string(), &game_id, &game_players)
            else {
                return Err(Error::from_str(500, "Update Row failed"));
            };

            // If there isnt an owning player we make the next player that joins the owning player.
            // TODO add a hook so that if the owning player quits it will pick someone else in the game
            match server_info.owning_player {
                Some(_) => {}
                None => update_row.database_data.push(PureDatabaseData {
                    data: serde_json::to_string(&request.request.player_id).unwrap(),
                    column_name: "owning_player".to_string(),
                }),
            }

            let _ = update_row_channel.send(update_row);
        }
    }
    tx.commit()?;

    Ok(tide::Response::builder(200).build())
}

/// A request to leave a game
pub struct QuitGameEndpoint {
    pub(crate) server_access_token: String,
    pub(crate) authentication_server_addr: Url,
    pub(crate) database: Database,
    pub(crate) update_row_channel: Sender<UpdateRow>,
}

#[async_trait]
impl Endpoint<()> for QuitGameEndpoint {
    async fn call(&self, req: Request<()>) -> tide::Result {
        quit_game(
            req,
            self.server_access_token.clone(),
            self.authentication_server_addr.clone(),
            self.database.clone(),
            self.update_row_channel.clone(),
        )
        .await
    }
}

struct QuitDbQuery {
    game_players: String,
}

/// Handles requests to quit a game
///
/// Verifies that the player is valid before it adds them to the game
async fn quit_game(
    mut req: Request<()>,
    access_token: String,
    auth_server_addr: Url,
    database: Database,
    update_row_channel: Sender<UpdateRow>,
) -> tide::Result {
    let request: HttpRequestMeta<QuitGame> = req.body_json().await?;
    auth_user_request(access_token.clone(), auth_server_addr.clone()).await?;

    // Simple verification that the player is in the game and can quit
    let mut connection = database.connection.lock().await;
    let tx = connection.transaction()?;
    {
        let mut stmt = tx.prepare(&format!(
            "SELECT game_players FROM games_meta where game_id = {}",
            request.request.game_id.to_json(),
        ))?;

        let server = stmt.query_map((), |row| {
            Ok(QuitDbQuery {
                game_players: row.get(1)?,
            })
        })?;

        for server in server {
            let server_info = server?;
            let mut game_players =
                match serde_json::from_str::<GamePlayers>(&server_info.game_players) {
                    Ok(info) => info,
                    Err(err) => return Err(Error::from_str(500, err)),
                };

            if !game_players.contains(&request.request.player_id) {
                return Err(Error::from_str(500, "Player already not in game"));
            }

            game_players.remove(&request.request.player_id);
            let Ok(update_row) = UpdateRow::new(
                "games_meta".to_string(),
                &request.request.game_id,
                &game_players,
            ) else {
                return Err(Error::from_str(500, "Update Row failed"));
            };

            let _ = update_row_channel.send(update_row);
        }
    }
    tx.commit()?;

    Ok(tide::Response::builder(200).build())
}
