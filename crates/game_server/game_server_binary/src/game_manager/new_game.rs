//! System that generates a new game

use bevy::{
    app::Plugin,
    ecs::{
        entity::Entity,
        world::{Mut, World},
    },
};
use core_library::{
    game_generation::create_game_world,
    game_meta::GameId,
    game_meta::NewGameSettings,
    objects::ObjectIdService,
    sqlite_database::schemes::game_server::{
        game_tables::{CreateGameCurvesTable, CreateGamePlayersTable},
        games_meta::InsertGamesMetaRow,
    },
    AsyncChannel, PendingDatabaseData,
};

use bevy::ecs::system::Command;

use super::{GameIdMapping, GameInstance};

pub struct NewGamePlugin;

impl Plugin for NewGamePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {}
}

/// Fn which generates a new game world, spawning it, and setting its initial state to the correct set and then returns the id of the game world's entity
pub fn generate_new_game(
    main_world: &mut World,
    settings: NewGameSettings,
    new_game_id: GameId,
    id_service: &mut ObjectIdService,
) -> Entity {
    let mut game_world = create_game_world(&settings, id_service);
    game_world.insert_resource(id_service.clone());
    let entity = main_world
        .spawn(GameInstance {
            game_id: new_game_id,
            game_world,
            future_actions: vec![],
            game_tick: super::GameTickInfo {
                game_tick: 0,
                ticks_per_tick: settings.ticks_per_tick,
                simulation_tick_amount: settings.simulation_tick_amount,
                last_simulated_tick: 0,
            },
        })
        .id();
    main_world.resource_scope(
        |_world: &mut World, mut game_id_mapping: Mut<GameIdMapping>| {
            game_id_mapping.map.insert(new_game_id.clone(), entity);
        },
    );

    entity
}

/// Command to create a new game
///
/// Must be infallible
pub struct NewGameCommand {
    pub new_game_settings: NewGameSettings,
    pub new_game_id: GameId,
}

impl Command for NewGameCommand {
    fn apply(self, world: &mut bevy::prelude::World) {
        let max_players = self.new_game_settings.max_player_count.clone();
        let mut id_service = ObjectIdService::new();
        let game_id = generate_new_game(
            world,
            self.new_game_settings,
            self.new_game_id,
            &mut id_service,
        );
        world.resource_scope(|_world: &mut World, mut mapping: Mut<GameIdMapping>| {
            mapping.map.insert(self.new_game_id, game_id)
        });

        world.resource_scope(
            |_world: &mut World, channel: Mut<AsyncChannel<InsertGamesMetaRow>>| {
                let _ = channel.sender_channel.send(InsertGamesMetaRow {
                    game_id: self.new_game_id.clone(),
                    max_players,
                    object_id_service: id_service,
                });
            },
        );
        world.resource_scope(
            |_world: &mut World, channel: Mut<AsyncChannel<CreateGameCurvesTable>>| {
                let _ = channel.sender_channel.send(CreateGameCurvesTable {
                    game_id: self.new_game_id.clone(),
                });
            },
        );
        world.resource_scope(
            |_world: &mut World, channel: Mut<AsyncChannel<CreateGamePlayersTable>>| {
                let _ = channel.sender_channel.send(CreateGamePlayersTable {
                    game_id: self.new_game_id.clone(),
                });
            },
        );
    }
}

/// Contains the neccessary information to create all the games required tables
struct NewGameInfo {}
