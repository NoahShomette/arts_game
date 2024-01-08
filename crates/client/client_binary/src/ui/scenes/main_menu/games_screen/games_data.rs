use bevy::{
    app::Plugin,
    //ecs::system::{Commands, Res, ResMut},
};
/*
use core_library::{
    authentication::{client_authentication::ClientAuthenticationInfo, AuthenticationServerInfo},
    AsyncChannel, TaskPoolRes,
};
use ehttp::Response;
*/
pub struct GamesDataPlugin;

impl Plugin for GamesDataPlugin {
    fn build(&self, _app: &mut bevy::prelude::App) {}
}
/*
struct RequestOpenGames {
    response: Result<Response, (u8, String)>,
}

fn request_open_games(
    client_info: Res<ClientAuthenticationInfo>,
    auth_server_info: Res<AuthenticationServerInfo>,
    task_pool_res: Res<TaskPoolRes>,
    check_username_channel: ResMut<AsyncChannel<RequestOpenGames>>,
) {
    /*
    let addr = auth_server_info.addr.clone();

    let check_username_channel = check_username_channel.clone();
    let client_info = client_info.clone();

    let message = match serde_json::to_string(&HttpRequestMeta {
        request: SetPlayerUsernameRequest {
            username: username.username.clone(),
        },
    }) {
        Ok(message) => message.as_bytes().to_vec(),
        Err(_) => {
            return;
        }
    };

    if let Some(task) = async_runners::run_async(
        async move {
            let mut request = ehttp::Request::post(format!("{}player/set_username", addr), message);

            request
                .headers
                .insert("Content-Type".to_string(), "application/json".to_string());
            request.headers.insert(
                "authorization".to_string(),
                format!("Bearer {}", client_info.sign_in_info.access_token.clone()),
            );
            match ehttp::fetch_async(request).await {
                Ok(response) => {
                    let _ = check_username_channel
                        .sender_channel
                        .send(SetUsernameResponse {
                            response: Ok(response),
                        });
                }
                Err(err) => {
                    let _ = check_username_channel
                        .sender_channel
                        .send(SetUsernameResponse { response: Err(err) });
                }
            };
        },
        &task_pool_res.0,
    ) {
        task.detach();
    }
    */
}

fn handle_set_username_responses(
    check_username_responses: ResMut<AsyncChannel<RequestOpenGames>>,
    mut commands: Commands,
) {
    if let Ok(receiver) = check_username_responses.reciever_channel.try_lock() {
        if let Ok(message) = receiver.try_recv() {
            if let Ok(message) = message.response {
                if message.status != 200 {
                    println!("Error: {}", message.status_text);
                    return;
                }

                let result_text = match message.text() {
                    Some(body) => body,
                    None => {
                        println!("No body");
                        return;
                    }
                };

                /*
                match serde_json::from_str::<SetPlayerUsernameResponse>(result_text) {
                    Ok(result) => match result {
                        SetPlayerUsernameResponse::Ok { new_username: _ } => {
                            commands.insert_resource(PlayerHasCustomUsername);
                            commands.run_system(systems.cleanup);
                            commands.run_system(systems.setup);
                        }
                        SetPlayerUsernameResponse::Error { error_text } => {
                            username_resource.error_text = error_text
                        }
                    },
                    Err(err) => {
                        println!("err: {} - on body {}", err, result_text);
                    }
                };
                */
            };
        }
    }
}
*/
