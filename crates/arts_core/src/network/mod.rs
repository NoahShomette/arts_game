use ehttp::Response;
use serde::{Deserialize, Serialize};

pub mod client_authentication;

/// A wrapper that contains meta information that the client sends to the Auth Server in order to correctly make requests
#[derive(Serialize, Deserialize)]
pub struct ClientHttpRequest<T> {
    pub access_token: Option<String>,
    pub request: T,
}

impl TryFrom<String> for SignInResponse {
    type Error = serde_json::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        serde_json::from_str::<SignInResponse>(&value)
    }
}

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
    pub user_metadata: String,
    pub identities: Vec<Identity>,
}

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
