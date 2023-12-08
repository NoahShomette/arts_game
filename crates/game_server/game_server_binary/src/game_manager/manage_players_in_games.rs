//! Responsible for handling players Joining and Leaving games.

use bevy_eventwork::async_trait;
use core_library::network::{game_http::JoinGame, HttpRequestMeta};
use tide::{http::Url, Endpoint, Request};

use crate::app_authentication::auth_user_request;

use super::game_database::DatabaseConnection;

/// A request to start a new game
pub struct JoinGameEndpoint {
    pub(crate) server_access_token: String,
    pub(crate) authentication_server_addr: Url,
    pub(crate) database: DatabaseConnection,
}

#[async_trait]
impl Endpoint<()> for JoinGameEndpoint {
    async fn call(&self, req: Request<()>) -> tide::Result {
        join_game(
            req,
            self.server_access_token.clone(),
            self.authentication_server_addr.clone(),
            self.database.clone(),
        )
        .await
    }
}

/// Handles requests to join a game
///
/// Verifies that the player is valid before it does so
async fn join_game(
    mut req: Request<()>,
    access_token: String,
    auth_server_addr: Url,
    database: DatabaseConnection,
) -> tide::Result {
    let request: HttpRequestMeta<JoinGame> = req.body_json().await?;
    auth_user_request(access_token.clone(), auth_server_addr.clone()).await?;

    Ok(tide::Response::builder(200).build())
}
