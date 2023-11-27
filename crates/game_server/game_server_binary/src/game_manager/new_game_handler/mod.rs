//! Responsible for communicating between auth server and client and setting up any new games

use arts_core::{
    authentication::AuthenticationServerInfo, http_server::TideServerResource, TaskPoolRes,
};
use bevy::{app::Plugin, ecs::world::Mut};

use self::requests::RequestNewGame;

pub mod requests;

pub struct NewGameHandlerPlugin;

impl Plugin for NewGameHandlerPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.world
            .resource_scope(|world, mut tide: Mut<TideServerResource>| {
                world.resource_scope(|world, mut auth: Mut<AuthenticationServerInfo>| {
                    tide.0.at("/games/request_new_game").get(RequestNewGame {
                        authentication_server_addr: auth.addr.clone(),
                    });
                });
            });
    }
}
