//! Responsible for establishing and managing active games. Each game gets their own [`GameInstance`]
//! which holds all their state between the [`GameInstance`] and the stuff in the actual world.
//!
//! Is also responsible though `crate::game_manager::save_manager` with saving each game into the server files

use bevy::{
    app::Plugin,
    ecs::{
        entity::Entity,
        schedule::{IntoSystemConfigs, OnEnter},
        system::Resource,
    },
    utils::HashMap,
};
use core_library::{authentication::AppAuthenticationState, game_meta::GameId};

use crate::http_network::start_server;

use self::{
    client_game_connection::ClientGameConnectionPlugin, game_database::GameDatabasePlugin,
    manage_players_in_games::add_join_and_quit_request, new_game::NewGamePlugin,
    new_game_http::NewGameHttpPlugin,
};

pub mod client_game_connection;
mod game_database;
mod manage_players_in_games;
mod new_game;
mod new_game_http;

pub struct GameManagerPlugin;

impl Plugin for GameManagerPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_resource(GameIdMapping {
            map: HashMap::new(),
        });
        app.add_plugins((
            GameDatabasePlugin,
            NewGameHttpPlugin,
            NewGamePlugin,
            ClientGameConnectionPlugin,
        ));

        app.add_systems(
            OnEnter(AppAuthenticationState::Authenticated),
            add_join_and_quit_request.before(start_server),
        );
    }
}

/// A resource that contains mappings from [`GameId`] -> [`Entity`]. This allows easy access to any specific game
#[derive(Resource)]
pub struct GameIdMapping {
    pub map: HashMap<GameId, Entity>,
}
