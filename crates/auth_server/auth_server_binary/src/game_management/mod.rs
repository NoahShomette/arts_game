use bevy::{app::Plugin, ecs::world::Mut};
use core_library::http_server::TideServerResource;

mod requests;

use core_library::sqlite_database::Database;

use self::requests::{RequestNewGame, RequestOpenGames};

pub struct GameManagementPlugin;

impl Plugin for GameManagementPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.world.resource_scope(|world, database: Mut<Database>| {
            world.resource_scope(|_world, mut tide: Mut<TideServerResource>| {
                tide.0.at("/games/request_new_game").post(RequestNewGame {
                    database: database.clone(),
                });
                tide.0
                    .at("/games/get_open_games/:offset")
                    .get(RequestOpenGames {
                        database: database.clone(),
                    });
            });
        });
    }
}
