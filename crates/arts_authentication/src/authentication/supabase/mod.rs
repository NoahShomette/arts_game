pub mod errors;

use std::{
    future::Future,
    sync::{
        mpsc::{self, Receiver, Sender},
        Arc, Mutex,
    },
};

use arts_core::client_authentication::{Password, RefreshToken};
use bevy::prelude::Resource;
use ureq::{Agent, Response};

use self::errors::{AuthErrors, AuthOk};

#[derive(Clone, Debug, Resource)]
pub struct Supabase {
    pub client: Agent,
    pub url: String,
    pub api_key: String,
    pub jwt: String,
    pub bearer_token: Option<String>,
    pub sender_channel: Sender<Result<AuthOk, AuthErrors>>,
    pub reciever_channel: Arc<Mutex<Receiver<Result<AuthOk, AuthErrors>>>>,
}

impl Supabase {
    // Creates a new Supabase client. If no parameters are provided, it will attempt to read the
    // environment variables `SUPABASE_URL`, `SUPABASE_API_KEY`, and `SUPABASE_JWT_SECRET`.
    pub fn new(url: Option<&str>, api_key: Option<&str>, jwt: Option<&str>) -> Self {
        let client: Agent = Agent::new();
        let url: String = url
            .map(String::from)
            .unwrap_or_else(|| String::from("https://xoyzqprxsavttcikuzop.supabase.co"));
        let api_key: String = api_key
            .map(String::from)
            .unwrap_or_else(|| String::from("eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpc3MiOiJzdXBhYmFzZSIsInJlZiI6InhveXpxcHJ4c2F2dHRjaWt1em9wIiwicm9sZSI6ImFub24iLCJpYXQiOjE2OTg4OTI3MjIsImV4cCI6MjAxNDQ2ODcyMn0.9-MFn6TrIAqp2OzyZuE_HpsSkR2soLskc8RuDDkS3PI"));
        let jwt: String = jwt
            .map(String::from)
            .unwrap_or_else(|| String::from("3xnIs6qo7gfAgXZmuTedQ2Ozk9U0sanOYNKlaQT+XlASCYPc6h8cGrM7KiAChZLJZf8HSayYrXqHXBMxECMMlw=="));

        let (sender, reciever) = mpsc::channel::<Result<AuthOk, AuthErrors>>();

        Supabase {
            client,
            url: url.to_string(),
            api_key: api_key.to_string(),
            jwt: jwt.to_string(),
            bearer_token: None,
            sender_channel: sender,
            reciever_channel: Arc::new(Mutex::new(reciever)),
        }
    }

    pub async fn sign_up_password(&self, sign_in_info: Password) -> impl Future<Output = ()> {
        let supabase = self.clone();
        async move {
            let request_url: String = format!("{}/auth/v1/signup", supabase.url);
            match supabase
                .client
                .post(&request_url)
                .set("apikey", &supabase.api_key.clone())
                .set("Content-Type", "application/json")
                .send_json(&sign_in_info)
            {
                Ok(response) => {
                    match response.into_string() {
                        Ok(response) => {
                            let _ = supabase.sender_channel.send(Ok(AuthOk::SignUp(response)));
                        }
                        Err(err) => {
                            let _ = supabase
                                .sender_channel
                                .send(Err(AuthErrors::Basic(format!("{}", err))));
                            return;
                        }
                    };
                }
                Err(err) => {
                    let _ = supabase
                        .sender_channel
                        .send(Err(AuthErrors::Basic(format!("{}", err))));
                    return;
                }
            }
        }
    }

    pub async fn sign_in_password(
        &self,
        sign_in_info: Password,
    ) -> Result<Response, crate::authentication::supabase::errors::AuthErrors> {
        let request_url: String = format!("{}/auth/v1/token?grant_type=password", self.url);
        match self
            .client
            .post(&request_url)
            .set("apikey", &self.api_key.clone())
            .set("Content-Type", "application/json")
            .send_json(&sign_in_info)
        {
            Ok(response) => {
                return Ok(response);
            }
            Err(err) => {
                return Err(AuthErrors::Basic(format!("{}", err)));
            }
        }
    }

    pub async fn refresh_token(&self, refresh: RefreshToken) -> impl Future<Output = ()> {
        let supabase: Supabase = self.clone();
        async move {
            let request_url: String =
                format!("{}/auth/v1/token?grant_type=refresh_token", supabase.url);
            match supabase
                .client
                .post(&request_url)
                .set("apikey", &supabase.api_key.clone())
                .set("Content-Type", "application/json")
                .send_json(&refresh)
            {
                Ok(response) => {
                    match response.into_string() {
                        Ok(response) => {
                            let _ = supabase
                                .sender_channel
                                .send(Ok(AuthOk::RefreshToken(response)));
                        }
                        Err(err) => {
                            let _ = supabase
                                .sender_channel
                                .send(Err(AuthErrors::Basic(format!("{}", err))));
                            return;
                        }
                    };
                }
                Err(err) => {
                    let _ = supabase
                        .sender_channel
                        .send(Err(AuthErrors::Basic(format!("{}", err))));
                    return;
                }
            }
        }
    }

    pub async fn logout(&self) -> impl Future<Output = ()> {
        let supabase: Supabase = self.clone();
        async move {
            let request_url: String = format!("{}/auth/v1/logout", supabase.url);
            let Some(bearer_token) = supabase.bearer_token else {
                let _ = supabase
                    .sender_channel
                    .send(Err(AuthErrors::Basic(format!("Invalid Bearer Token"))));
                return;
            };
            match supabase
                .client
                .post(&request_url)
                .set("apikey", &supabase.api_key.clone())
                .set("Content-Type", "application/json")
                .set("autherization", &format!("Bearer {}", bearer_token))
                .call()
            {
                Ok(response) => {
                    match response.into_string() {
                        Ok(response) => {
                            let _ = supabase.sender_channel.send(Ok(AuthOk::SignOut(response)));
                        }
                        Err(err) => {
                            let _ = supabase
                                .sender_channel
                                .send(Err(AuthErrors::Basic(format!("{}", err))));
                            return;
                        }
                    };
                }
                Err(err) => {
                    let _ = supabase
                        .sender_channel
                        .send(Err(AuthErrors::Basic(format!("{}", err))));
                    return;
                }
            }
        }
    }
}
