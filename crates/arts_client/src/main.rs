use arts_client::{AuthClient, ClientResponses};
use arts_core::async_runners;
use arts_core::network::client_authentication::Password;
use arts_core::network::{ClientHttpRequest, SignInResponse};
use bevy::app::App;
use bevy::ecs::system::Commands;
use bevy::prelude::{Res, Resource, Startup, Update};
use bevy::tasks::{TaskPool, TaskPoolBuilder};
use bevy::DefaultPlugins;
use ehttp::Response;
use serde_json::Value;

fn main() {
    let mut app = App::new();
    app.insert_resource(AuthClient::new());
    app.add_plugins(DefaultPlugins);
    app.insert_resource(TaskPoolRes(TaskPoolBuilder::new().num_threads(2).build()));
    app.add_systems(Startup, sign_in);
    app.add_systems(Update, (receive_auth_results, logout));
    app.run();
}

#[derive(Resource)]
pub struct TaskPoolRes(pub TaskPool);

#[derive(Resource, Clone)]
pub struct ClientConnectionInfo {
    access_token: String,
}

fn sign_in(auth_client: Res<AuthClient>, task_pool_res: Res<TaskPoolRes>, mut commands: Commands) {
    let supabase = auth_client.clone();
    if let Some(task) = async_runners::run_async(
        async move {
            let mut request = ehttp::Request::post(
                "http://127.0.0.1:2030/auth/sign_in",
                serde_json::to_string(&ClientHttpRequest {
                    access_token: None,
                    request: Password {
                        email: "noahshomette@gmail.com".to_string(),
                        password: "123456789".to_string(),
                    },
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

fn logout(
    supabase: Res<AuthClient>,
    task_pool_res: Res<TaskPoolRes>,
    client_info: Option<Res<ClientConnectionInfo>>,
) {
    if let Some(client) = client_info {
        println!("We are getting into this");
        let client = client.clone();
        let supabase = supabase.clone();
        if let Some(task) = async_runners::run_async(
            async move {
                let mut request = ehttp::Request::post(
                    "http://127.0.0.1:2030/auth/logout",
                    serde_json::to_string(&ClientHttpRequest {
                        access_token: Some(client.access_token.clone()),
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
                        let _ = supabase
                            .sender_channel
                            .send(Ok((ClientResponses::LogOut, response)));
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

fn receive_auth_results(supa: Res<AuthClient>, commands: Commands) {
    let Ok(channel) = supa.reciever_channel.lock() else {
        return;
    };

    let result = channel.try_recv();

    if result.is_ok() {
        handle_response_data(result.unwrap().unwrap(), commands);
    };
}

fn handle_response_data(response: (ClientResponses, Response), mut commands: Commands) {
    match response.0 {
        ClientResponses::SignIn => {
            let v: Value = serde_json::from_str(response.1.text().unwrap()).unwrap();
            commands.insert_resource(ClientConnectionInfo {
                access_token: v["access_token"].to_string(),
            });
            println!("Logged in")
        }
        ClientResponses::LogOut => {
            commands.remove_resource::<ClientConnectionInfo>();
            println!("{}", format!("Logged Out: {}", response.1.text().unwrap()));
        }
    }
}
