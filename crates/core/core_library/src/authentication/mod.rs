use bevy::ecs::schedule::common_conditions::in_state;
use bevy::{
    app::{Plugin, Update},
    ecs::{
        schedule::{IntoSystemConfigs, States},
        system::Resource,
    },
};
use serde::{Deserialize, Serialize};
use url::Url;

use self::client_authentication::AuthClient;
use self::{
    client_auth_systems::{receive_auth_results, sign_in, sign_out, sign_up},
    client_authentication::{SignInEvent, SignOutEvent, SignUpEvent},
};

mod client_auth_systems;
pub mod client_authentication;

pub struct CoreAuthenticationPlugin;

impl Plugin for CoreAuthenticationPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_resource(AuthClient::new());
        app.init_resource::<AuthenticationServerInfo>();
        app.add_state::<AppAuthenticationState>();
        app.add_event::<SignInEvent>()
            .add_event::<SignOutEvent>()
            .add_event::<SignUpEvent>();
        app.add_systems(
            Update,
            (
                sign_in.run_if(in_state(AppAuthenticationState::NotAuthenticated)),
                receive_auth_results,
                sign_out,
                sign_up.run_if(in_state(AppAuthenticationState::NotAuthenticated)),
            ),
        );
    }
}

/// The current Authentication state of the app. Basically whether the app has logged in or not
#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum AppAuthenticationState {
    #[default]
    NotAuthenticated,
    Authenticated,
}

#[derive(Resource)]
pub struct AuthenticationServerInfo {
    pub addr: Url,
}

impl Default for AuthenticationServerInfo {
    fn default() -> Self {
        Self {
            addr: Url::parse("http://127.0.0.1:2030").unwrap(),
        }
    }
}

/// The response returned from the server when a user logins.
#[derive(Serialize, Deserialize)]
pub struct SignUpResponse {
    pub user: UserInfo,
}

/// The response returned from the server when a user logins.
#[derive(Serialize, Deserialize)]
pub struct SignInResponse {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: u32,
    pub expires_at: u32,
    pub refresh_token: String,
    pub user: UserInfo,
}

#[derive(Serialize, Deserialize)]
pub struct UserInfo {
    pub id: String,
    pub aud: String,
    pub role: String,
    pub email: String,
    pub email_confirmed_at: Option<String>,
    pub phone: Option<String>,
    pub confirmation_sent_at: String,
    pub confirmed_at: Option<String>,
    pub last_sign_in_at: String,
    pub app_metadata: AppMetadata,
    pub user_metadata: UserMetadata,
    pub identities: Vec<Identity>,
}

#[derive(Serialize, Deserialize)]
pub struct UserMetadata {}

#[derive(Serialize, Deserialize)]
pub struct AppMetadata {
    pub provider: String,
    pub providers: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Identity {
    pub id: String,
    pub user_id: String,
    pub identity_data: IdentityData,
    pub provider: String,
    pub last_sign_in_at: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Serialize, Deserialize)]
pub struct IdentityData {
    pub email: String,
    pub sub: String,
}
