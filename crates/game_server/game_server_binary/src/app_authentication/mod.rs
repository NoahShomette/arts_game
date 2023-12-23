//! Is responsible for authenticating the server. Eventually the server will require an account that has the Server role. This
//! will allow the server to also make http requests to change data for the data it owns. Eg delete a game, change a game ip, etc.

use bevy::app::Plugin;
use ehttp::Response;
use tide::{http::Url, Error};

pub struct AuthenticationPlugin;

impl Plugin for AuthenticationPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugins(core_library::authentication::CoreAuthenticationPlugin);
    }
}

/// Simple request that returns Ok if the access token is valid
pub async fn auth_user_request(
    access_token: String,
    auth_server_addr: Url,
) -> tide::Result<Response> {
    let mut request = ehttp::Request::get(format!("{}auth/authenticate_user", auth_server_addr));

    request
        .headers
        .insert("Content-Type".to_string(), "application/json".to_string());

    request.headers.insert(
        "authorization".to_string(),
        format!("Bearer {}", access_token),
    );

    match ehttp::fetch_async(request).await {
        Ok(response) => Ok(response),
        Err(err) => Err(Error::from_str(500, err)),
    }
}
