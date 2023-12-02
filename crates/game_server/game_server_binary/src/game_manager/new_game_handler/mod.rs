//! Responsible for communicating between auth server and client and setting up any new games
//!
//! Receives requests from clients to start a new game. It then gets a new game id from the auth server, which registers it on the server, and then it starts the new game
//! sends that info to the auth server

use bevy::{app::Plugin, ecs::world::Mut};
use core_library::{
    authentication::{client_authentication::ClientAuthenticationInfo, AuthenticationServerInfo},
    http_server::TideServerResource,
    network::GameAddrInfo,
};

use self::{new_game_command::NewGameCommandsChannel, requests::RequestNewGame};

pub mod new_game_command;
pub mod requests;

pub struct NewGameHandlerPlugin;

impl Plugin for NewGameHandlerPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_resource(NewGameCommandsChannel::new());
        app.world
            .resource_scope(|world, mut tide: Mut<TideServerResource>| {
                world.resource_scope(|world, auth: Mut<AuthenticationServerInfo>| {
                    world.resource_scope(|world, client: Mut<ClientAuthenticationInfo>| {
                        world.resource_scope(|world, game: Mut<GameAddrInfo>| {
                            world.resource_scope(|_world, channel: Mut<NewGameCommandsChannel>| {
                                tide.0.at("/games/request_new_game").get(RequestNewGame {
                                    authentication_server_addr: auth.addr.clone(),
                                    access_token: client.sign_in_info.access_token.clone(),
                                    self_server_id: client.sign_in_info.user.id.clone(),
                                    game_ip: game.clone(),
                                    channel: channel.clone(),
                                });
                            });
                        });
                    });
                });
            });
    }
}
