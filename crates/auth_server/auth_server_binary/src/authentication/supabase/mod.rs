pub mod errors;

use std::sync::{
    mpsc::{self, Receiver, Sender},
    Arc, Mutex,
};

use arts_core::authentication::client_authentication::{Claims, PasswordLoginInfo, RefreshToken};
use bevy::prelude::Resource;
use ehttp::Response;
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use serde::{Deserialize, Serialize};

use self::errors::{AuthErrors, AuthOk};

#[derive(Serialize, Deserialize)]
struct InternalPasswordLoginInfo {
    email: String,
    password: String,
}

impl From<PasswordLoginInfo> for InternalPasswordLoginInfo {
    fn from(value: PasswordLoginInfo) -> Self {
        Self {
            email: value.email().to_string(),
            password: value.password().to_string(),
        }
    }
}

/// A resource containing meta info connecting the auth server to the supabase instance and communicating supabase interactions in and out via an MPSC
#[derive(Clone, Debug, Resource)]
pub struct SupabaseConnection {
    pub url: String,
    pub api_key: String,
    pub jwt: String,
    pub sender_channel: Sender<Result<AuthOk, AuthErrors>>,
    pub reciever_channel: Arc<Mutex<Receiver<Result<AuthOk, AuthErrors>>>>,
}

impl SupabaseConnection {
    /// Internal function on the server to validate if a JWT is a valid token or not
    ///
    /// # NOTE
    ///
    /// This does not validate if the token sent gives authorization to access any requested data
    pub fn jwt_valid(&self, jwt: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
        let secret = self.jwt.clone();

        let decoding_key = DecodingKey::from_secret(secret.as_ref()).into();
        let mut validation = Validation::new(Algorithm::HS256);
        validation.set_audience(&["authenticated"]);
        let decoded_token = decode::<Claims>(jwt, &decoding_key, &validation);

        match decoded_token {
            Ok(token_data) => {
                println!("Token is valid. Claims: {:?}", token_data.claims);
                Ok(token_data.claims)
            }
            Err(err) => {
                println!("Error decoding token: {:?}", err);
                Err(err)
            }
        }
    }

    // Creates a new Supabase client. If no parameters are provided, it will attempt to read the
    // environment variables `SUPABASE_URL`, `SUPABASE_API_KEY`, and `SUPABASE_JWT_SECRET`.
    pub fn new(url: Option<&str>, api_key: Option<&str>) -> Self {
        let url: String = url
            .map(String::from)
            .unwrap_or_else(|| String::from("https://xoyzqprxsavttcikuzop.supabase.co"));
        let api_key: String = api_key
            .map(String::from)
            .unwrap_or_else(|| String::from("eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpc3MiOiJzdXBhYmFzZSIsInJlZiI6InhveXpxcHJ4c2F2dHRjaWt1em9wIiwicm9sZSI6ImFub24iLCJpYXQiOjE3MDAxMDkzMjksImV4cCI6MjAxNTY4NTMyOX0.CtoLhyZp58ZEi6yzOdMGh5H-oBTVe1MJ1iqtrxdoCtY"));

        let jwt_secret =
            dotenv::var("JWT_SECRET").expect("Must have JWT_SECRET when starting server");

        let jwt: String = jwt_secret;

        let (sender, reciever) = mpsc::channel::<Result<AuthOk, AuthErrors>>();

        SupabaseConnection {
            url: url.to_string(),
            api_key: api_key.to_string(),
            jwt: jwt,
            sender_channel: sender,
            reciever_channel: Arc::new(Mutex::new(reciever)),
        }
    }

    pub async fn sign_up_password(
        &self,
        sign_up_info: PasswordLoginInfo,
    ) -> Result<Response, crate::authentication::supabase::errors::AuthErrors> {
        let request_url: String = format!("{}/auth/v1/signup", self.url);

        let mut request = ehttp::Request::post(
            request_url,
            serde_json::to_string(&InternalPasswordLoginInfo::from(sign_up_info))
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

    pub async fn sign_in_password(
        &self,
        sign_in_info: PasswordLoginInfo,
    ) -> Result<Response, crate::authentication::supabase::errors::AuthErrors> {
        let request_url: String = format!("{}/auth/v1/token?grant_type=password", self.url);

        let mut request = ehttp::Request::post(
            request_url,
            serde_json::to_string(&InternalPasswordLoginInfo::from(sign_in_info))
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
        access_token: String,
    ) -> Result<Response, crate::authentication::supabase::errors::AuthErrors> {
        let request_url: String = format!("{}/auth/v1/logout", self.url);

        let mut request = ehttp::Request::get(request_url);

        request
            .headers
            .insert("apikey".to_string(), self.api_key.clone());
        request
            .headers
            .insert("Content-Type".to_string(), "application/json".to_string());
        request.headers.insert(
            "authorization".to_string(),
            format!("Bearer {}", access_token),
        );

        match ehttp::fetch_async(request).await {
            Ok(response) => Ok(response),
            Err(err) => Err(AuthErrors::Basic(format!("{}", err))),
        }
    }
}
