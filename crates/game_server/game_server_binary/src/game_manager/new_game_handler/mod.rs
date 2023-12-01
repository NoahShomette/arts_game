//! Responsible for communicating between auth server and client and setting up any new games

use arts_core::{
    authentication::{client_authentication::ClientAuthenticationInfo, AuthenticationServerInfo},
    http_server::TideServerResource,
    network::GameAddrInfo,
};
use bevy::{app::Plugin, ecs::world::Mut};

use self::requests::RequestNewGame;

pub mod requests;

pub struct NewGameHandlerPlugin;

impl Plugin for NewGameHandlerPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.world
            .resource_scope(|world, mut tide: Mut<TideServerResource>| {
                world.resource_scope(|world, auth: Mut<AuthenticationServerInfo>| {
                    world.resource_scope(|world, client: Mut<ClientAuthenticationInfo>| {
                        world.resource_scope(|world, game: Mut<GameAddrInfo>| {
                            tide.0.at("/games/request_new_game").get(RequestNewGame {
                                authentication_server_addr: auth.addr.clone(),
                                access_token: client.sign_in_info.access_token.clone(),
                                self_server_id: client.sign_in_info.user.id.clone(),
                                game_ip: game.clone(),
                            });
                        });
                    });
                });
            });
    }
}
