use core_library::http_server::TideServerResource;
use bevy::{app::Plugin, ecs::world::Mut};

mod requests;

use crate::database::Database;

use self::requests::RequestNewGame;

pub struct GameManagementPlugin;

impl Plugin for GameManagementPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.world.resource_scope(|world, database: Mut<Database>| {
            world.resource_scope(|_world, mut tide: Mut<TideServerResource>| {
                tide.0
                    .at("/game_management/request_new_game")
                    .post(RequestNewGame {
                        database: database.clone(),
                    });
            });
        });
    }
}
