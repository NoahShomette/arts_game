use bevy::ecs::event::{EventReader, EventWriter};
use bevy::ecs::schedule::NextState;
use bevy::ecs::system::{Commands, ResMut, Resource};
use bevy::prelude::Res;
use ehttp::Response;

use crate::authentication::SignInResponse;
use crate::network::ClientHttpRequest;
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

pub fn signup(
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
        if let Some(task) = async_runners::run_async(
            async move {
                let mut request = ehttp::Request::post(
                    format!("{}/auth/signup", addr),
                    serde_json::to_string(&ClientHttpRequest {
                        access_token: None,
                        request: login.info.clone(),
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

pub fn signin(
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

        if let Some(task) = async_runners::run_async(
            async move {
                let mut request = ehttp::Request::post(
                    format!("{}/auth/signin", addr),
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

pub fn signout(
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

            if let Some(task) = async_runners::run_async(
                async move {
                    let mut request = ehttp::Request::post(
                        format!("{}/auth/signout", addr),
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
    commands: Commands,
    app_state: ResMut<NextState<AppAuthenticationState>>,
    login_events: EventWriter<SignInEvent>,
) {
    let Ok(channel) = supa.reciever_channel.lock() else {
        return;
    };

    let result = channel.try_recv();

    if let Ok(result) = result {
        match result {
            Ok(result) => handle_response_data(result, commands, app_state, login_events),
            Err(err) => println!("Error {}", err),
        };
    };
}

pub fn handle_response_data(
    response: (AuthenticationResponses, Response),
    mut commands: Commands,
    mut app_state: ResMut<NextState<AppAuthenticationState>>,
    mut login_events: EventWriter<SignInEvent>,
) {
    match response.0 {
        AuthenticationResponses::SignIn => {
            commands.remove_resource::<TryingToSignIn>();
            let v: SignInResponse = serde_json::from_str(response.1.text().unwrap()).unwrap();
            commands.insert_resource(ClientAuthenticationInfo {
                access_token: v.access_token.clone(),
            });
            app_state.set(AppAuthenticationState::Authenticated);
            println!("Logged in")
        }
        AuthenticationResponses::SignOut => {
            commands.remove_resource::<ClientAuthenticationInfo>();
            commands.remove_resource::<TryingToSignOut>();
            app_state.set(AppAuthenticationState::NotAuthenticated);
            println!("Logged Out");
        }
        AuthenticationResponses::SignUp(login_info) => {
            commands.remove_resource::<TryingToSignUp>();
            login_events.send(SignInEvent {
                login_info: login_info,
            });
            println!("Signed Up")
        }
    }
}
