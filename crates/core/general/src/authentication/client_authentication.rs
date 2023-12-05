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
pub struct PasswordLoginInfo {
    is_player: bool,
    email: String,
    password: String,
}

impl PasswordLoginInfo {
    pub fn new(email: &str, password: &str, is_player: bool) -> PasswordLoginInfo {
        PasswordLoginInfo {
            email: email.to_string(),
            password: Base64::encode_string(&Sha3_256::digest(password)),
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
}

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
    pub reciever_channel: Arc<Mutex<Receiver<Result<(AuthenticationResponses, Response), String>>>>,
}

impl AuthClient {
    pub fn new() -> Self {
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
    pub login_info: PasswordLoginInfo,
}

/// An event sent to sign up
#[derive(Event, Clone)]
pub struct SignUpEvent {
    pub info: PasswordLoginInfo,
}

/// An event sent to logout
#[derive(Event, Clone)]
pub struct SignOutEvent;

pub enum AuthenticationResponses {
    SignIn,
    SignOut,
    SignUp(PasswordLoginInfo),
}
