use std::sync::Arc;

use arts_core::authentication::client_authentication::PasswordLoginInfo;
use arts_core::network::ClientHttpRequest;
use tide::utils::async_trait;
use tide::{Endpoint, Error, Request};

use super::supabase::SupabaseConnection;

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
    let request: ClientHttpRequest<PasswordLoginInfo> = req.body_json().await?;
    let result = supabase.sign_in_password(request.request).await?;
    Ok(tide::Response::builder(200)
        .body(result.text().unwrap())
        .build())
}

/// A request to log out
pub struct Logout {
    pub(crate) supabase: Arc<SupabaseConnection>,
}

#[async_trait]
impl Endpoint<()> for Logout {
    async fn call(&self, req: Request<()>) -> tide::Result {
        logout(req, &self.supabase).await
    }
}

async fn logout(mut req: Request<()>, supabase: &SupabaseConnection) -> tide::Result {
    let request: ClientHttpRequest<()> = req.body_json().await?;
    let Some(access_token) = request.access_token else {
        return Err(Error::from_str(400, "Access Token Required to Logout"));
    };
    let result = supabase.logout(access_token).await?;
    Ok(tide::Response::builder(200)
        .body(result.text().unwrap())
        .build())
}
