use bevy::log::info;
use core_library::auth_server::player_data::PlayerGames;
use core_library::auth_server::AccountId;

use core_library::authentication::client_authentication::{
    EmailPasswordCredentials, RefreshTokenRequest,
};
use core_library::authentication::{SignInResponse, SignUpResponse};
use core_library::http_server::request_access_token;
use core_library::network::HttpRequestMeta;
use core_library::sqlite_database::Database;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tide::utils::async_trait;
use tide::{Endpoint, Error, Request};
use uuid::Uuid;

use crate::user_management::verify_decode_jwt;

use super::supabase::SupabaseConnection;

/// A request to sign up
pub struct SignUp {
    pub(crate) supabase: Arc<SupabaseConnection>,
    pub(crate) database: Database,
}

#[async_trait]
impl Endpoint<()> for SignUp {
    async fn call(&self, req: Request<()>) -> tide::Result {
        sign_up(req, &self.supabase, &self.database).await
    }
}

async fn sign_up(
    mut req: Request<()>,
    supabase: &SupabaseConnection,
    database: &Database,
) -> tide::Result {
    info!("Received Sign Up Request");
    let request: HttpRequestMeta<EmailPasswordCredentials> = req.body_json().await?;
    let is_player = request.request.is_player();
    let result = supabase.sign_up_password(request.request).await?;

    let mut connection = database.connection.lock().await;
    if let Some(result_text) = result.text() {
        let v: SignUpResponse = serde_json::from_str(result_text)?;
        let _user_id = v.id;
        let account_id = serde_json::to_string(&AccountId { id: _user_id })?;
        let tx = connection.transaction()?;

        match is_player {
            true => {
                {
                    // Check if there is already a player account saved
                    match tx.prepare(&format!(
                        "SELECT player_id FROM player_data where player_id = {}",
                        account_id
                    )) {
                        Ok(_) => {}
                        Err(_) => {
                            let _ = tx.execute(
                                "insert into player_data (player_id, player_games, username) values (?1, ?2, ?3)",
                                [
                                    &account_id,
                                    &serde_json::to_string(&PlayerGames {
                                        current_games: vec![],
                                    })
                                    .expect("Creates a default PlayerGames. Should always be valid"),
                                    &Uuid::new_v4().to_string(),
                                ],
                            );
                        }
                    }
                }
            }
            false => {
                {
                    // Check if there is already a server account saved
                    match tx.prepare(&format!(
                        "SELECT server_id FROM server_data where server_id = {}",
                        account_id
                    )) {
                        Ok(_) => {}
                        Err(_) => {
                            let _ = tx.execute(
                                "insert into server_data (server_id, server_type) values (?1, ?2)",
                                [
                                    &account_id,
                                    &serde_json::to_string(&0)
                                        .expect("Serializing 0 should always be valid"),
                                ],
                            );
                        }
                    }
                }
            }
        }
        tx.commit()?;
    }
    match result.text() {
        Some(body) => Ok(tide::Response::builder(200).body(body).build()),
        None => Err(Error::from_str(500, "Failed to sign up")),
    }
}

/// A request to sign in
pub struct SignIn {
    pub(crate) supabase: Arc<SupabaseConnection>,
}

#[async_trait]
impl Endpoint<()> for SignIn {
    async fn call(&self, req: Request<()>) -> tide::Result {
        sign_in(req, &self.supabase).await
    }
}

async fn sign_in(mut req: Request<()>, supabase: &SupabaseConnection) -> tide::Result {
    info!("Received Sign In Request");
    let request: HttpRequestMeta<EmailPasswordCredentials> = req.body_json().await?;
    let result = supabase.sign_in_password(request.request).await?;
    match result.text() {
        Some(body) => Ok(tide::Response::builder(200).body(body).build()),
        None => Err(Error::from_str(500, "Failed to sign in")),
    }
}

/// A request to log out
pub struct SignOut {
    pub(crate) supabase: Arc<SupabaseConnection>,
}

#[async_trait]
impl Endpoint<()> for SignOut {
    async fn call(&self, req: Request<()>) -> tide::Result {
        signout(req, &self.supabase).await
    }
}

async fn signout(req: Request<()>, supabase: &SupabaseConnection) -> tide::Result {
    println!("Received Sign Out Request");
    // Verify that it is a real user who is validly signed in
    let _ = verify_decode_jwt(&req, supabase)?;
    let access_token = request_access_token(&req)?;
    let result = supabase.logout(access_token).await?;
    match result.text() {
        Some(body) => Ok(tide::Response::builder(200).body(body).build()),
        None => Err(Error::from_str(500, "Failed to sign out")),
    }
}

/// Authenticates a user as validly signed in
pub struct AuthenticateUser {
    pub(crate) supabase: Arc<SupabaseConnection>,
}

#[async_trait]
impl Endpoint<()> for AuthenticateUser {
    async fn call(&self, req: Request<()>) -> tide::Result {
        authenticate_user(req, &self.supabase).await
    }
}

async fn authenticate_user(req: Request<()>, supabase: &SupabaseConnection) -> tide::Result {
    println!("Received Authentication Request");
    // Verify that it is a real user who is validly signed in
    let claims = verify_decode_jwt(&req, supabase)?;

    match serde_json::to_string(&claims) {
        Ok(claims) => Ok(tide::Response::builder(200).body(claims.as_str()).build()),
        Err(err) => Err(Error::from_str(
            500,
            format!("Failed to serialize Claims with error: {}", err),
        )),
    }
}

/// Refreshes a users access token.
pub struct RefreshTokenEndpoint {
    pub(crate) supabase: Arc<SupabaseConnection>,
}

#[async_trait]
impl Endpoint<()> for RefreshTokenEndpoint {
    async fn call(&self, req: Request<()>) -> tide::Result {
        refresh_user(req, &self.supabase).await
    }
}

async fn refresh_user(mut req: Request<()>, supabase: &SupabaseConnection) -> tide::Result {
    println!("Received Sign Out Request");
    // Verify that it is a real user who is validly signed in
    let _ = verify_decode_jwt(&req, supabase)?;
    let request: HttpRequestMeta<RefreshTokenRequest> = req.body_json().await?;

    let result = supabase
        .refresh_token(request.request.refresh_token)
        .await?;
    match result.text() {
        Some(body) => Ok(tide::Response::builder(200).body(body).build()),
        None => Err(Error::from_str(500, "Failed to refresh user")),
    }
}

/// Refreshes a users access token.
pub struct IsUserEmailConfirmed {
    pub(crate) supabase: Arc<SupabaseConnection>,
}

#[async_trait]
impl Endpoint<()> for IsUserEmailConfirmed {
    async fn call(&self, req: Request<()>) -> tide::Result {
        check_if_user_email_confirmed(req, &self.supabase).await
    }
}

async fn check_if_user_email_confirmed(
    mut req: Request<()>,
    supabase: &SupabaseConnection,
) -> tide::Result {
    println!("Received email confirmation request");
    let request: HttpRequestMeta<
        core_library::authentication::client_authentication::IsUserEmailConfirmed,
    > = req.body_json().await?;

    let result = supabase.sign_in_password(request.request.info).await?;

    if !result.status == 200 {
        return Err(Error::from_str(500, "Failed to verify user"));
    }

    let result_text = match result.text() {
        Some(body) => body,
        None => return Err(Error::from_str(500, "Failed to verify user")),
    };

    // Check if the user is verified or not. If we cant deserialize into the error struct than we assume its a valid login for now
    let is_verified = match serde_json::from_str::<SignInResponse>(result_text) {
        Ok(response) => {
            let _ = supabase.logout(response.access_token).await?;
            true
        }
        Err(_) => false,
    };

    Ok(tide::Response::builder(200)
        .body(is_verified.to_string())
        .build())
}

#[derive(Serialize, Deserialize)]
struct BasicUserDetails {
    error: String,
    error_description: String,
}
