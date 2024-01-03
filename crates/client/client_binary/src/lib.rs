mod network;
pub mod ui;

use app::AppPlugin;
use asset_loading::AssetLoadingPlugin;
use authentication::AuthenticationPlugin;
use bevy::ecs::system::{Commands, Res, Resource};
use bevy::{app::Plugin, tasks::TaskPoolBuilder};
use camera::CameraPlugin;
use core_library::authentication::client_authentication::ClientAuthenticationInfo;
use core_library::game_meta::NewGameSettings;
use core_library::network::{GameAddrInfo, HttpRequestMeta};
use core_library::{async_runners, TaskPoolRes};
use network::NetworkPlugin;
use ui::ClientUiPlugin;

pub mod app;
pub mod asset_loading;
mod authentication;
pub mod camera;
pub mod player;

pub struct ClientPlugin;

impl Plugin for ClientPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugins((
            AppPlugin,
            AssetLoadingPlugin,
            NetworkPlugin,
            ClientUiPlugin,
            CameraPlugin,
            AuthenticationPlugin,
        ));

        app.insert_resource(GameAddrInfo {
            server_addr: "127.0.0.1".to_string(),
            http_port: 2031,
            ws_port: 2032,
        });

        app.insert_resource(TaskPoolRes(TaskPoolBuilder::new().num_threads(2).build()));
    }
}

#[derive(Resource)]
struct RequestedNewGame;

#[allow(dead_code)]
fn new_game(
    task_pool_res: Res<TaskPoolRes>,
    game_server_info: Res<GameAddrInfo>,
    client_info: Res<ClientAuthenticationInfo>,
    g: Option<Res<RequestedNewGame>>,
    mut commands: Commands,
) {
    if g.is_some() {
        return;
    }
    commands.insert_resource(RequestedNewGame);
    let new_game_settings = NewGameSettings {
        max_player_count: 10,
        map_point_count: core_library::game_meta::MapPointCount::Dense,
        map_size: core_library::game_meta::MapSize::Large,
        connection_density: core_library::game_meta::ConnectionDensity::Dense,
        ticks_per_tick: 1,
        simulation_tick_amount: 1,
        game_name: String::from("New game testing"),
    };

    let addr = game_server_info.http_url();
    let message = match serde_json::to_string(&HttpRequestMeta {
        request: new_game_settings,
    }) {
        Ok(message) => message.as_bytes().to_vec(),
        Err(_err) => {
            return;
        }
    };
    let client_info = client_info.clone();

    if let Some(task) = async_runners::run_async(
        async move {
            let mut request =
                ehttp::Request::post(format!("{}games/request_new_game", addr), message);

            request
                .headers
                .insert("Content-Type".to_string(), "application/json".to_string());

            request.headers.insert(
                "authorization".to_string(),
                format!("Bearer {}", client_info.sign_in_info.access_token.clone()),
            );
            match ehttp::fetch_async(request).await {
                Ok(response) => {
                    println!("response: {:?}", response)
                }
                Err(err) => {
                    println!("err: {}", err)
                }
            };
        },
        &task_pool_res.0,
    ) {
        task.detach();
    }
}
