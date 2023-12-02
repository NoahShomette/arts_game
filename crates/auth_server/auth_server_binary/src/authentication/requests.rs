use std::sync::Arc;

use bevy::log::info;
use core_library::auth_server::player_data::PlayerGames;
use core_library::authentication::client_authentication::PasswordLoginInfo;
use core_library::authentication::SignInResponse;
use core_library::network::HttpRequestMeta;
use tide::utils::async_trait;
use tide::{Endpoint, Request};

use crate::database::Database;
use crate::user_management::{request_access_token, verify_decode_jwt};

use super::supabase::SupabaseConnection;

/// A request to sign up
pub struct SignUp {
    pub(crate) supabase: Arc<SupabaseConnection>,
}

#[async_trait]
impl Endpoint<()> for SignUp {
    async fn call(&self, req: Request<()>) -> tide::Result {
        sign_up(req, &self.supabase).await
    }
}

async fn sign_up(mut req: Request<()>, supabase: &SupabaseConnection) -> tide::Result {
    info!("Received Sign Up Request");
    let request: HttpRequestMeta<PasswordLoginInfo> = req.body_json().await?;
    let result = supabase.sign_up_password(request.request).await?;
    Ok(tide::Response::builder(200)
        .body(result.text().unwrap())
        .build())
}

/// A request to sign in
pub struct SignIn {
    pub(crate) supabase: Arc<SupabaseConnection>,
    pub(crate) database: Database,
}

#[async_trait]
impl Endpoint<()> for SignIn {
    async fn call(&self, req: Request<()>) -> tide::Result {
        sign_in(req, &self.supabase, &self.database).await
    }
}

async fn sign_in(
    mut req: Request<()>,
    supabase: &SupabaseConnection,
    database: &Database,
) -> tide::Result {
    info!("Received Sign In Request");
    let request: HttpRequestMeta<PasswordLoginInfo> = req.body_json().await?;
    let is_player = request.request.is_player();
    let result = supabase.sign_in_password(request.request).await?;
    if let Ok(mut connection) = database.connection.lock() {
        let tx = connection.transaction()?;
        if let Some(result_text) = result.text() {
            let v: SignInResponse = serde_json::from_str(result_text)?;
            let _user_id = v.user.id.clone();
            match is_player {
                true => {
                    let _ = tx.execute(
                        "insert into player_data (player_id, player_games) values (?1, ?2)",
                        &[
                            &_user_id,
                            &serde_json::to_string(&PlayerGames {
                                current_games: vec![],
                            })
                            .unwrap(),
                        ],
                    );
                }
                false => {
                    let _ = tx.execute(
                        "insert into server_data (server_id, server_type) values (?1, ?2)",
                        &[&_user_id, &serde_json::to_string(&0).unwrap()],
                    );
                }
            }
        }
        tx.commit()?;
    }
    Ok(tide::Response::builder(200)
        .body(result.text().unwrap())
        .build())
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
    Ok(tide::Response::builder(200)
        .body(result.text().unwrap())
        .build())
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
    println!("Received Sign Out Request");
    // Verify that it is a real user who is validly signed in
    let _ = verify_decode_jwt(&req, supabase)?;
    Ok(tide::Response::builder(200).build())
}
