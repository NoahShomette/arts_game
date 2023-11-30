use bevy::ecs::event::{EventReader, EventWriter};
use bevy::ecs::schedule::NextState;
use bevy::ecs::system::{Commands, ResMut, Resource};
use bevy::log::info;
use bevy::prelude::Res;
use ehttp::Response;

use crate::authentication::SignInResponse;
use crate::network::HttpRequestMeta;
use crate::{async_runners, TaskPoolRes};

use super::client_authentication::{
    AuthClient, AuthenticationResponses, ClientAuthenticationInfo, SignInEvent, SignOutEvent,
    SignUpEvent,
};
use super::{AppAuthenticationState, AuthenticationServerInfo};

#[derive(Resource)]
pub struct TryingToSignUp;

#[derive(Resource)]
pub struct TryingToSignOut;

#[derive(Resource)]
pub struct TryingToSignIn;

pub fn sign_up(
    auth_client: Res<AuthClient>,
    task_pool_res: Res<TaskPoolRes>,
    ttl: Option<Res<TryingToSignUp>>,
    mut commands: Commands,
    mut login_events: EventReader<SignUpEvent>,
    auth_server_info: Res<AuthenticationServerInfo>,
) {
    if ttl.is_some() {
        return;
    }
    for sign_up in login_events.read() {
        commands.insert_resource(TryingToSignUp);
        let supabase = auth_client.clone();
        let login = sign_up.clone();
        let addr = auth_server_info.addr.clone();
        let message = match serde_json::to_string(&HttpRequestMeta {
            request: login.info.clone(),
        }) {
            Ok(message) => message.as_bytes().to_vec(),
            Err(err) => {
                let _ = supabase.sender_channel.send(Err(format!("{}", err)));
                return;
            }
        };
        if let Some(task) = async_runners::run_async(
            async move {
                let mut request = ehttp::Request::post(format!("{}auth/sign_up", addr), message);
                request
                    .headers
                    .insert("Content-Type".to_string(), "application/json".to_string());

                match ehttp::fetch_async(request).await {
                    Ok(response) => {
                        let _ = supabase
                            .sender_channel
                            .send(Ok((AuthenticationResponses::SignUp(login.info), response)));
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

pub fn sign_in(
    auth_client: Res<AuthClient>,
    task_pool_res: Res<TaskPoolRes>,
    ttl: Option<Res<TryingToSignIn>>,
    mut commands: Commands,
    mut login_events: EventReader<SignInEvent>,
    auth_server_info: Res<AuthenticationServerInfo>,
) {
    if ttl.is_some() {
        return;
    }
    for login in login_events.read() {
        commands.insert_resource(TryingToSignIn);
        let supabase = auth_client.clone();
        let login = login.clone();
        let addr = auth_server_info.addr.clone();
        let message = match serde_json::to_string(&HttpRequestMeta {
            request: login.login_info.clone(),
        }) {
            Ok(message) => message.as_bytes().to_vec(),
            Err(err) => {
                let _ = supabase.sender_channel.send(Err(format!("{}", err)));
                return;
            }
        };
        if let Some(task) = async_runners::run_async(
            async move {
                let mut request = ehttp::Request::post(format!("{}auth/sign_in", addr), message);

                request
                    .headers
                    .insert("Content-Type".to_string(), "application/json".to_string());

                match ehttp::fetch_async(request).await {
                    Ok(response) => {
                        let _ = supabase
                            .sender_channel
                            .send(Ok((AuthenticationResponses::SignIn, response)));
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

pub fn sign_out(
    auth_client: Res<AuthClient>,
    task_pool_res: Res<TaskPoolRes>,
    client_info: Option<Res<ClientAuthenticationInfo>>,
    ttl: Option<Res<TryingToSignOut>>,
    mut commands: Commands,
    mut login_events: EventReader<SignOutEvent>,
    auth_server_info: Res<AuthenticationServerInfo>,
) {
    if ttl.is_some() {
        return;
    }
    if let Some(client) = client_info {
        for _ in login_events.read() {
            commands.insert_resource(TryingToSignOut);
            let client_info = client.clone();
            let auth_client = auth_client.clone();
            let addr = auth_server_info.addr.clone();
            let message = match serde_json::to_string(&HttpRequestMeta { request: () }) {
                Ok(message) => message.as_bytes().to_vec(),
                Err(err) => {
                    let _ = auth_client.sender_channel.send(Err(format!("{}", err)));
                    return;
                }
            };
            if let Some(task) = async_runners::run_async(
                async move {
                    let mut request =
                        ehttp::Request::post(format!("{}auth/sign_out", addr), message);

                    request
                        .headers
                        .insert("Content-Type".to_string(), "application/json".to_string());

                    request.headers.insert(
                        "autherization".to_string(),
                        format!("Bearer {}", client_info.access_token.clone()),
                    );

                    match ehttp::fetch_async(request).await {
                        Ok(response) => {
                            let _ = auth_client
                                .sender_channel
                                .send(Ok((AuthenticationResponses::SignOut, response)));
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

pub fn receive_auth_results(
    supa: Res<AuthClient>,
    mut commands: Commands,
    app_state: ResMut<NextState<AppAuthenticationState>>,
) {
    let Ok(channel) = supa.reciever_channel.lock() else {
        return;
    };

    let result = channel.try_recv();

    commands.remove_resource::<TryingToSignIn>();
    commands.remove_resource::<TryingToSignOut>();
    commands.remove_resource::<TryingToSignUp>();

    if let Ok(result) = result {
        match result {
            Ok(result) => match (result.1.ok, result.1.status) {
                (true, 200) => handle_response_data(result, commands, app_state),
                (_, _) => info!(
                    "Error Code {} : Status Text '{}'",
                    result.1.status, result.1.status_text
                ),
            },
            Err(err) => info!("Error {}", err),
        };
    };
}

pub fn handle_response_data(
    response: (AuthenticationResponses, Response),
    mut commands: Commands,
    mut app_state: ResMut<NextState<AppAuthenticationState>>,
) {
    match response.0 {
        AuthenticationResponses::SignIn => {
            let v: SignInResponse = serde_json::from_str(response.1.text().unwrap()).unwrap();
            commands.insert_resource(ClientAuthenticationInfo {
                access_token: v.access_token.clone(),
            });
            app_state.set(AppAuthenticationState::Authenticated);
            println!("Logged in")
        }
        AuthenticationResponses::SignOut => {
            commands.remove_resource::<ClientAuthenticationInfo>();
            app_state.set(AppAuthenticationState::NotAuthenticated);
            println!("Logged Out");
        }
        AuthenticationResponses::SignUp(login_info) => {
            println!("Signed Up");
        }
    }
}
