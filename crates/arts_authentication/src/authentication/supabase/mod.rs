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
use ehttp::Response;

use self::errors::{AuthErrors, AuthOk};

#[derive(Clone, Debug, Resource)]
pub struct Supabase {
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
            url: url.to_string(),
            api_key: api_key.to_string(),
            jwt: jwt.to_string(),
            bearer_token: None,
            sender_channel: sender,
            reciever_channel: Arc::new(Mutex::new(reciever)),
        }
    }

    pub async fn sign_up_password(&self, sign_up_info: Password) -> impl Future<Output = ()> {
        let supabase = self.clone();
        async move {
            let request_url: String = format!("{}/auth/v1/signup", supabase.url);

            let mut request = ehttp::Request::post(
                request_url,
                serde_json::to_string(&sign_up_info)
                    .unwrap()
                    .as_bytes()
                    .to_vec(),
            );

            request
                .headers
                .insert("apikey".to_string(), supabase.api_key.clone());
            request
                .headers
                .insert("Content-Type".to_string(), "application/json".to_string());

            ehttp::fetch(request, move |result| {
                match result {
                    Ok(response) => match response.text() {
                        Some(text) => {
                            let _ = supabase
                                .sender_channel
                                .send(Ok(AuthOk::SignUp(text.to_owned())));
                        }
                        None => {
                            let _ = supabase.sender_channel.send(Err(AuthErrors::Basic(format!(
                                "{}: Response did not include a Body",
                                response.status_text
                            ))));
                            return;
                        }
                    },
                    Err(err) => {
                        let _ = supabase
                            .sender_channel
                            .send(Err(AuthErrors::Basic(format!("{}", err))));
                        return;
                    }
                };
            });
        }
    }

    pub async fn sign_in_password(
        &self,
        sign_in_info: Password,
    ) -> Result<Response, crate::authentication::supabase::errors::AuthErrors> {
        let request_url: String = format!("{}/auth/v1/token?grant_type=password", self.url);

        let mut request = ehttp::Request::post(
            request_url,
            serde_json::to_string(&sign_in_info)
                .unwrap()
                .as_bytes()
                .to_vec(),
        );

        request
            .headers
            .insert("apikey".to_string(), self.api_key.clone());
        request
            .headers
            .insert("Content-Type".to_string(), "application/json".to_string());

        match ehttp::fetch_async(request).await {
            Ok(response) => Ok(response),
            Err(err) => Err(AuthErrors::Basic(format!("{}", err))),
        }
    }

    pub async fn refresh_token(
        &self,
        refresh: RefreshToken,
    ) -> Result<Response, crate::authentication::supabase::errors::AuthErrors> {
        let request_url: String = format!("{}/auth/v1/token?grant_type=refresh_token", self.url);

        let mut request = ehttp::Request::post(
            request_url,
            serde_json::to_string(&refresh).unwrap().as_bytes().to_vec(),
        );

        request
            .headers
            .insert("apikey".to_string(), self.api_key.clone());
        request
            .headers
            .insert("Content-Type".to_string(), "application/json".to_string());

        match ehttp::fetch_async(request).await {
            Ok(response) => Ok(response),
            Err(err) => Err(AuthErrors::Basic(format!("{}", err))),
        }
    }

    pub async fn logout(
        &self,
    ) -> Result<Response, crate::authentication::supabase::errors::AuthErrors> {
        let request_url: String = format!("{}/auth/v1/logout", self.url);

        let mut request = ehttp::Request::get(request_url);

        let Some(bearer_token) = self.bearer_token.clone() else {
            let _ = self
                .sender_channel
                .send(Err(AuthErrors::Basic(format!("Invalid Bearer Token"))));
            return Err(AuthErrors::Basic(format!("Invalid Bearer Token")));
        };

        request
            .headers
            .insert("apikey".to_string(), self.api_key.clone());
        request
            .headers
            .insert("Content-Type".to_string(), "application/json".to_string());
        request.headers.insert(
            "autherization".to_string(),
            format!("Bearer {}", bearer_token),
        );

        match ehttp::fetch_async(request).await {
            Ok(response) => Ok(response),
            Err(err) => Err(AuthErrors::Basic(format!("{}", err))),
        }
    }
}
