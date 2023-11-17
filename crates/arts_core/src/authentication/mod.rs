use bevy::ecs::schedule::States;
use serde::{Deserialize, Serialize};

pub mod client_authentication;

/// The current Authentication state of the app. Basically whether the app has logged in or not
#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum AppAuthenticationState {
    #[default]
    NotAuthenticated,
    Authenticated,
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
