use bevy::{
    app::{Plugin, Update},
    ecs::{
        change_detection::DetectChanges,
        schedule::{common_conditions::in_state, IntoSystemConfigs, OnEnter},
        system::{Res, ResMut},
    },
    utils::{hashbrown::HashMap, Instant},
};

use core_library::{
    async_runners,
    auth_server::game::RequestOpenGamesResponse,
    authentication::{client_authentication::ClientAuthenticationInfo, AuthenticationServerInfo},
    game_meta::GameId,
    network::{
        game_http::{GamesInfoResponse, RequestGamesInfo},
        HttpRequestMeta,
    },
    AsyncChannel, TaskPoolRes,
};
use ehttp::Response;
use url::Url;

use crate::ui::scenes::main_menu::MainMenuScreenState;

use super::{LastRequestInfo, OpenGamesData};

pub struct GamesDataPlugin;

impl Plugin for GamesDataPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_resource(AsyncChannel::<RequestOpenGamesChannel>::default());
        app.insert_resource(AsyncChannel::<RequestGameServerInfoChannel>::default());
        app.add_systems(OnEnter(MainMenuScreenState::GamesPage), request_open_games);
        app.add_systems(
            Update,
            (
                handle_games_data_responses,
                request_game_information_from_game_server,
                handle_game_server_responses,
            )
                .run_if(in_state(MainMenuScreenState::GamesPage)),
        );
    }
}

#[derive(Clone)]
struct RequestOpenGamesChannel {
    response: Result<Response, String>,
}

#[derive(Clone)]
struct RequestGameServerInfoChannel {
    response: Result<Response, String>,
}

fn request_open_games(
    client_info: Res<ClientAuthenticationInfo>,
    auth_server_info: Res<AuthenticationServerInfo>,
    task_pool_res: Res<TaskPoolRes>,
    channel: ResMut<AsyncChannel<RequestOpenGamesChannel>>,
) {
    let addr = auth_server_info.addr.clone();

    let check_username_channel = channel.clone();
    let client_info = client_info.clone();
    let offset = 0;
    if let Some(task) = async_runners::run_async(
        async move {
            let mut request =
                ehttp::Request::get(format!("{}games/get_open_games/{}", addr, offset));

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
                        .send(RequestOpenGamesChannel {
                            response: Ok(response),
                        });
                }
                Err(err) => {
                    let _ = check_username_channel
                        .sender_channel
                        .send(RequestOpenGamesChannel { response: Err(err) });
                }
            };
        },
        &task_pool_res.0,
    ) {
        task.detach();
    }
}

fn request_game_information_from_game_server(
    mut open_games_res: ResMut<OpenGamesData>,
    client_info: Res<ClientAuthenticationInfo>,
    task_pool_res: Res<TaskPoolRes>,
    channel: ResMut<AsyncChannel<RequestGameServerInfoChannel>>,
) {
    if !open_games_res.is_changed() {
        return;
    }

    let mut games_to_get_info: HashMap<Url, Vec<GameId>> = HashMap::new();

    for (id, game) in open_games_res.games.iter_mut() {
        if game.1.is_some() || game.2.is_some() {
            continue;
        }

        let addr = game.0.game_ip.http_url();
        let entry = games_to_get_info.entry(addr).or_insert(vec![]);
        entry.push(*id);
        game.2 = Some(LastRequestInfo {
            request_time: Instant::now(),
        })
    }

    for (addr, games) in games_to_get_info.iter() {
        let check_username_channel = channel.clone();
        let client_info = client_info.clone();
        let addr = addr.clone();
        let message = match serde_json::to_string(&HttpRequestMeta {
            request: RequestGamesInfo {
                games: games.clone(),
            },
        }) {
            Ok(message) => message.as_bytes().to_vec(),
            Err(_) => {
                return;
            }
        };
        if let Some(task) = async_runners::run_async(
            async move {
                let mut request =
                    ehttp::Request::post(format!("{}games/games_info", addr), message);

                request
                    .headers
                    .insert("Content-Type".to_string(), "application/json".to_string());
                request.headers.insert(
                    "authorization".to_string(),
                    format!("Bearer {}", client_info.sign_in_info.access_token.clone()),
                );
                match ehttp::fetch_async(request).await {
                    Ok(response) => {
                        let _ = check_username_channel.sender_channel.send(
                            RequestGameServerInfoChannel {
                                response: Ok(response),
                            },
                        );
                    }
                    Err(err) => {
                        let _ = check_username_channel
                            .sender_channel
                            .send(RequestGameServerInfoChannel { response: Err(err) });
                    }
                };
            },
            &task_pool_res.0,
        ) {
            task.detach();
        }
    }
}

fn handle_games_data_responses(
    check_username_responses: ResMut<AsyncChannel<RequestOpenGamesChannel>>,
    mut open_games_res: ResMut<OpenGamesData>,
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

                match serde_json::from_str::<RequestOpenGamesResponse>(result_text) {
                    Ok(result) => {
                        for game in result.games.iter() {
                            open_games_res
                                .games
                                .insert(game.game_id, (game.clone(), None, None));
                        }
                    }
                    Err(err) => {
                        println!("err: {} - on body {}", err, result_text);
                    }
                };
            };
        }
    }
}

fn handle_game_server_responses(
    check_username_responses: ResMut<AsyncChannel<RequestGameServerInfoChannel>>,
    mut open_games_res: ResMut<OpenGamesData>,
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

                match serde_json::from_str::<GamesInfoResponse>(result_text) {
                    Ok(result) => {
                        for game in result.games.iter() {
                            let Some(entry) = open_games_res.games.get_mut(&game.game_id) else {
                                continue;
                            };
                            entry.1 = Some(game.clone());
                        }
                    }
                    Err(err) => {
                        println!("err: {} - on body {}", err, result_text);
                    }
                };
            };
        }
    }
}
