use base64ct::{Base64, Encoding};
use serde::{Deserialize, Serialize};
use sha3::Digest;

use bevy::ecs::event::Event;
use bevy::prelude::Resource;
use ehttp::Response;
use sha3::Sha3_256;

use std::sync::{
    mpsc::{self, Receiver, Sender},
    Arc, Mutex,
};

use super::SignInResponse;

#[derive(Serialize, Deserialize, Clone)]
pub struct EmailPasswordCredentials {
    is_player: bool,
    email: String,
    password: String,
}

impl EmailPasswordCredentials {
    /// Creates a new set of credentials. Hashes the password
    pub fn new(email: &str, password: &str, is_player: bool) -> EmailPasswordCredentials {
        EmailPasswordCredentials {
            email: email.to_string(),
            password: hash_password(password),
            is_player,
        }
    }

    pub fn email(&self) -> &str {
        &self.email
    }

    pub fn password(&self) -> &str {
        &self.password
    }

    pub fn is_player(&self) -> bool {
        self.is_player
    }

    /// Creates a new credentials with a pre hashed password
    pub fn new_pre_hashed(
        email: &str,
        password: String,
        is_player: bool,
    ) -> EmailPasswordCredentials {
        EmailPasswordCredentials {
            email: email.to_string(),
            password,
            is_player,
        }
    }
}

/// Hashes a password. All passwords are hashed before signing in or signing up.
pub fn hash_password(password: &str) -> String {
    Base64::encode_string(&Sha3_256::digest(password))
}

/// A request sent by a user to check if they are confirmed
#[derive(Serialize, Deserialize, Clone)]
pub struct IsUserEmailConfirmed {
    pub info: EmailPasswordCredentials,
}

/// A request sent by a user to refresh their access token
#[derive(Serialize, Deserialize, Clone)]
pub struct RefreshTokenRequest {
    pub refresh_token: RefreshToken,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct RefreshToken {
    refresh_token: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub email: String,
    pub exp: usize,
}

impl Clone for Claims {
    fn clone(&self) -> Self {
        Self {
            sub: self.sub.clone(),
            email: self.email.clone(),
            exp: self.exp,
        }
    }
}

#[derive(Resource, Clone)]
pub struct AuthClient {
    pub sender_channel: Sender<Result<(AuthenticationResponses, Response), String>>,
    #[allow(clippy::type_complexity)]
    pub reciever_channel: Arc<Mutex<Receiver<Result<(AuthenticationResponses, Response), String>>>>,
}

impl Default for AuthClient {
    fn default() -> Self {
        let (sender, reciever) =
            mpsc::channel::<Result<(AuthenticationResponses, Response), String>>();

        AuthClient {
            sender_channel: sender,
            reciever_channel: Arc::new(Mutex::new(reciever)),
        }
    }
}

/// Resource inserted into the app when the app is signed in that holds the current authentication and client info
#[derive(Resource, Clone)]
pub struct ClientAuthenticationInfo {
    pub sign_in_info: SignInResponse,
}

/// An event sent to sign in
#[derive(Event, Clone)]
pub struct SignInEvent {
    pub login_info: EmailPasswordCredentials,
}

/// An event sent to sign up
#[derive(Event, Clone)]
pub struct SignUpEvent {
    pub info: EmailPasswordCredentials,
}

/// An event sent to logout
#[derive(Event, Clone)]
pub struct SignOutEvent;

pub enum AuthenticationResponses {
    SignIn,
    SignOut,
    SignUp(EmailPasswordCredentials),
}
