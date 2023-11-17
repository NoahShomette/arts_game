use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct PasswordLoginInfo {
    pub email: String,
    pub password: String,
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

use bevy::app::{Plugin, Update};
use bevy::ecs::event::{Event, EventReader};
use bevy::ecs::schedule::common_conditions::in_state;
use bevy::ecs::schedule::{IntoSystemConfigs, NextState};
use bevy::ecs::system::{Commands, ResMut};
use bevy::prelude::{Res, Resource};
use ehttp::Response;

use std::sync::{
    mpsc::{self, Receiver, Sender},
    Arc, Mutex,
};

use crate::authentication::SignInResponse;
use crate::network::ClientHttpRequest;
use crate::{async_runners, TaskPoolRes};

use super::AppAuthenticationState;

pub struct CoreAuthenticationPlugin;

impl Plugin for CoreAuthenticationPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_state::<AppAuthenticationState>();
        app.add_event::<LoginEvent>().add_event::<LogoutEvent>();
        app.add_systems(
            Update,
            (
                sign_in.run_if(in_state(AppAuthenticationState::NotAuthenticated)),
                receive_auth_results,
                logout,
            ),
        );
    }
}

/// An event sent to login
#[derive(Event, Clone)]
pub struct LoginEvent {
    login_info: PasswordLoginInfo,
}

/// An event sent to logout
#[derive(Event, Clone)]
pub struct LogoutEvent;

#[derive(Resource, Clone)]
pub struct AuthClient {
    pub sender_channel: Sender<Result<(ClientResponses, Response), String>>,
    pub reciever_channel: Arc<Mutex<Receiver<Result<(ClientResponses, Response), String>>>>,
}

impl AuthClient {
    pub fn new() -> Self {
        let (sender, reciever) = mpsc::channel::<Result<(ClientResponses, Response), String>>();

        AuthClient {
            sender_channel: sender,
            reciever_channel: Arc::new(Mutex::new(reciever)),
        }
    }
}

pub enum ClientResponses {
    SignIn,
    LogOut,
}

#[derive(Resource, Clone)]
pub struct ClientConnectionInfo {
    access_token: String,
}

#[derive(Resource)]
struct TryingToLogout;

#[derive(Resource)]
struct TryingToLogin;

fn sign_in(
    auth_client: Res<AuthClient>,
    task_pool_res: Res<TaskPoolRes>,
    ttl: Option<Res<TryingToLogin>>,
    mut commands: Commands,
    mut login_events: EventReader<LoginEvent>,
) {
    if ttl.is_some() {
        return;
    }
    for login in login_events.read() {
        commands.insert_resource(TryingToLogin);
        let supabase = auth_client.clone();
        let login = login.clone();
        if let Some(task) = async_runners::run_async(
            async move {
                let mut request = ehttp::Request::post(
                    "http://127.0.0.1:2030/auth/sign_in",
                    serde_json::to_string(&ClientHttpRequest {
                        access_token: None,
                        request: login.login_info,
                    })
                    .unwrap()
                    .as_bytes()
                    .to_vec(),
                );

                request
                    .headers
                    .insert("Content-Type".to_string(), "application/json".to_string());

                match ehttp::fetch_async(request).await {
                    Ok(response) => {
                        let _ = supabase
                            .sender_channel
                            .send(Ok((ClientResponses::SignIn, response)));
                    }
                    Err(err) => {
                        let _ = supabase.sender_channel.send(Err(format!("{}", err)));
                    }
                };
            },
            &task_pool_res.0,
        ) {
            task.detach();
        }
    }
}

fn logout(
    auth_client: Res<AuthClient>,
    task_pool_res: Res<TaskPoolRes>,
    client_info: Option<Res<ClientConnectionInfo>>,
    ttl: Option<Res<TryingToLogout>>,
    mut commands: Commands,
    mut login_events: EventReader<LogoutEvent>,
) {
    if ttl.is_some() {
        return;
    }
    if let Some(client) = client_info {
        for _ in login_events.read() {
            commands.insert_resource(TryingToLogout);
            let client_info = client.clone();
            let auth_client = auth_client.clone();
            if let Some(task) = async_runners::run_async(
                async move {
                    let mut request = ehttp::Request::post(
                        "http://127.0.0.1:2030/auth/logout",
                        serde_json::to_string(&ClientHttpRequest {
                            access_token: Some(client_info.access_token.clone()),
                            request: (),
                        })
                        .unwrap()
                        .as_bytes()
                        .to_vec(),
                    );

                    request
                        .headers
                        .insert("Content-Type".to_string(), "application/json".to_string());

                    match ehttp::fetch_async(request).await {
                        Ok(response) => {
                            let _ = auth_client
                                .sender_channel
                                .send(Ok((ClientResponses::LogOut, response)));
                        }
                        Err(err) => {
                            let _ = auth_client.sender_channel.send(Err(format!("{}", err)));
                        }
                    };
                },
                &task_pool_res.0,
            ) {
                task.detach();
            }
        }
    }
}

fn receive_auth_results(
    supa: Res<AuthClient>,
    commands: Commands,
    app_state: ResMut<NextState<AppAuthenticationState>>,
) {
    let Ok(channel) = supa.reciever_channel.lock() else {
        return;
    };

    let result = channel.try_recv();

    if result.is_ok() {
        handle_response_data(result.unwrap().unwrap(), commands, app_state);
    };
}

fn handle_response_data(
    response: (ClientResponses, Response),
    mut commands: Commands,
    mut app_state: ResMut<NextState<AppAuthenticationState>>,
) {
    match response.0 {
        ClientResponses::SignIn => {
            commands.remove_resource::<TryingToLogin>();
            let v: SignInResponse = serde_json::from_str(response.1.text().unwrap()).unwrap();
            commands.insert_resource(ClientConnectionInfo {
                access_token: v.access_token.clone(),
            });
            app_state.set(AppAuthenticationState::Authenticated);
            println!("Logged in")
        }
        ClientResponses::LogOut => {
            commands.remove_resource::<ClientConnectionInfo>();
            commands.remove_resource::<TryingToLogout>();
            app_state.set(AppAuthenticationState::NotAuthenticated);
            println!("Logged Out");
        }
    }
}
