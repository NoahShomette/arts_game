//! Responsible for communicating between auth server and client and setting up any new games
//!
//! Receives requests from clients to start a new game. It then gets a new game id from the auth server, which registers it on the server, and then it starts the new game
//! sends that info to the auth server

use bevy::{
    app::Plugin,
    ecs::{
        schedule::{IntoSystemConfigs, OnEnter},
        system::{Res, ResMut},
    },
};
use core_library::{
    authentication::{
        client_authentication::ClientAuthenticationInfo, AppAuthenticationState,
        AuthenticationServerInfo,
    },
    http_server::TideServerResource,
    network::GameAddrInfo,
};

use crate::http_network::start_server;

use self::requests::{NewGameCommandsChannel, RequestNewGame};

pub mod requests;

pub struct NewGameHttpPlugin;

impl Plugin for NewGameHttpPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_resource(NewGameCommandsChannel::new());
        app.add_systems(
            OnEnter(AppAuthenticationState::Authenticated),
            add_new_game_request_to_server.before(start_server),
        );
    }
}

fn add_new_game_request_to_server(
    mut tide: ResMut<TideServerResource>,
    auth: Res<AuthenticationServerInfo>,
    client: Res<ClientAuthenticationInfo>,
    game: Res<GameAddrInfo>,
    channel: Res<NewGameCommandsChannel>,
) {
    tide.0.at("/games/request_new_game").post(RequestNewGame {
        authentication_server_addr: auth.addr.clone(),
        access_token: client.sign_in_info.access_token.clone(),
        self_server_id: client.sign_in_info.user.id,
        game_ip: game.clone(),
        channel: channel.clone(),
    });
}
