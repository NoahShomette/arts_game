use self::client_authentication::{AuthClient, Claims};
use self::{
    client_auth_systems::{receive_auth_results, sign_in, sign_out, sign_up},
    client_authentication::{SignInEvent, SignOutEvent, SignUpEvent},
};
use bevy::ecs::schedule::common_conditions::in_state;
use bevy::{
    app::{Plugin, Update},
    ecs::{
        schedule::{IntoSystemConfigs, States},
        system::Resource,
    },
};
use serde::de::{self};
use serde::{Deserialize, Deserializer, Serialize};
use url::Url;
use uuid::Uuid;

mod client_auth_systems;
pub mod client_authentication;

pub struct CoreAuthenticationPlugin;

impl Plugin for CoreAuthenticationPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_resource(AuthClient::default());
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
                sign_out.run_if(in_state(AppAuthenticationState::Authenticated)),
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

/// The response returned from the server when a user signs up.
#[derive(Serialize, Deserialize)]
pub struct SignUpResponse {
    #[serde(deserialize_with = "into_uuid")]
    pub id: Uuid,
    pub aud: String,
    pub role: String,
    pub email: String,
    pub phone: Option<String>,
    pub confirmation_sent_at: String,
    pub app_metadata: AppMetadata,
    pub user_metadata: UserMetadata,
    pub identities: Vec<SignUpIdentity>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct SignUpIdentity {
    #[serde(deserialize_with = "into_uuid")]
    pub identity_id: Uuid,
    #[serde(deserialize_with = "into_uuid")]
    pub id: Uuid,
    #[serde(deserialize_with = "into_uuid")]
    pub user_id: Uuid,
    pub identity_data: IdentityData,
    pub provider: String,
    pub last_sign_in_at: String,
    pub created_at: String,
    pub updated_at: String,
    pub email: String,
}
/*
{"
id":"c3844b49-2d32-4f10-be06-3bd736b3b9c5",
"aud":"authenticated",
"role":"authenticated",
"email":"test@test.com",
"phone":"",
"confirmation_sent_at":"2023-12-23T02:26:23.138765327Z",
"app_metadata":
    {
        "provider":"email",
        "providers":["email"]
    },
"user_metadata":{},
"identities":[
    {"identity_id":"9296053f-bb65-43dc-ad41-ef657b5366ec",
    "id":"c3844b49-2d32-4f10-be06-3bd736b3b9c5",
    "user_id":"c3844b49-2d32-4f10-be06-3bd736b3b9c5",
    "identity_data":
    {
        "email":"test@test.com",
        "email_verified":false,
        "phone_verified":false,
        "sub":"c3844b49-2d32-4f10-be06-3bd736b3b9c5"
    },
    "provider":"email",
    "last_sign_in_at":"2023-12-23T02:26:23.134853854Z",
    "created_at":"2023-12-23T02:26:23.134924Z",
    "updated_at":"2023-12-23T02:26:23.134924Z",
    "email":"test@test.com"}
    ],
    "created_at":"2023-12-23T02:26:23.131977Z",
    "updated_at":"2023-12-23T02:26:23.420354Z"
}
*/

/// The response returned from the server when a user logins.
#[derive(Serialize, Deserialize, Clone)]
pub struct SignInResponse {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: u32,
    pub expires_at: u32,
    pub refresh_token: String,
    pub user: UserInfo,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct UserInfo {
    #[serde(deserialize_with = "into_uuid")]
    pub id: Uuid,
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

#[derive(Serialize, Deserialize, Clone)]
pub struct UserMetadata {}

#[derive(Serialize, Deserialize, Clone)]
pub struct AppMetadata {
    pub provider: String,
    pub providers: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Identity {
    #[serde(deserialize_with = "into_uuid")]
    pub id: Uuid,
    #[serde(deserialize_with = "into_uuid")]
    pub user_id: Uuid,
    pub identity_data: IdentityData,
    pub provider: String,
    pub last_sign_in_at: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct IdentityData {
    pub email: String,
    pub email_verified: bool,
    pub phone_verified: bool,
    #[serde(deserialize_with = "into_uuid")]
    pub sub: Uuid,
}

fn into_uuid<'de, D>(deserializer: D) -> Result<Uuid, D::Error>
where
    D: Deserializer<'de>,
{
    let s: &str = Deserialize::deserialize(deserializer)?;
    Uuid::parse_str(s).map_err(|_| de::Error::custom("Error parsing into UUID"))
}

/// Response sent from the auth server to any server requesting to authenticate and decode a users Access Token
pub struct AuthenticationRequestResponse {
    pub claims: Claims,
}
