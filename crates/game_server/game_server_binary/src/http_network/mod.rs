//! Responsible for all http server functions

use bevy::{
    app::Plugin,
    ecs::{
        schedule::{IntoSystemConfigs, OnEnter},
        world::World,
    },
};
use core_library::{authentication::AppAuthenticationState, http_server::TideServerResource};

use self::game_http::add_game_info_request;

mod game_http;

pub struct HttpNetworkPlugin;

impl Plugin for HttpNetworkPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(OnEnter(AppAuthenticationState::Authenticated), start_server);
        app.add_systems(
            OnEnter(AppAuthenticationState::Authenticated),
            add_game_info_request.before(start_server),
        );
    }
}

pub fn start_server(world: &mut World) {
    // Must be the last items called starting the server
    let tide = world
        .remove_resource::<TideServerResource>()
        .expect("TideServerResource expected to start server");
    tide.start_server();
}
