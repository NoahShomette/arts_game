//! System that generates a new game

use bevy::{
    app::{Plugin, Update},
    ecs::{
        entity::Entity,
        schedule::IntoSystemConfigs,
        system::{Commands, ResMut},
        world::{Mut, World},
    },
};
use core_library::{
    auth_server::AccountId,
    game_generation::{create_game_world, insert_new_game_state},
    game_meta::GameId,
    game_meta::NewGameSettings,
    objects::ObjectIdService,
    sqlite_database::schemes::game_server::{
        game_tables::{CreateGameCurvesTable, CreateGamePlayersTable},
        games_meta::InsertGamesMetaRow,
    },
    AsyncChannelSender,
};

use bevy::ecs::system::Command;

use crate::app::app_scheduling::ServerAuthenticatedSets;

use super::{new_game_http::requests::NewGameCommandsChannel, GameIdMapping, GameInstance};

pub struct NewGamePlugin;

impl Plugin for NewGamePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(
            Update,
            read_new_game_command_channel.in_set(ServerAuthenticatedSets::ServerTasks),
        );
    }
}

fn read_new_game_command_channel(
    new_game_commands: ResMut<NewGameCommandsChannel>,
    mut commands: Commands,
) {
    if let Ok(receiver) = new_game_commands.reciever_channel.try_lock() {
        while let Ok(new_game_commands) = receiver.try_recv() {
            commands.add(new_game_commands);
        }
    }
}

/// Fn which generates a new game world, spawning it, and setting its initial state to the correct set and then returns the id of the game world's entity
pub fn generate_new_game(
    server_world: &mut World,
    settings: NewGameSettings,
    new_game_id: GameId,
    id_service: &mut ObjectIdService,
) -> Entity {
    let mut game_world = create_game_world(server_world, &new_game_id, &settings, id_service);
    insert_new_game_state(&mut game_world, &new_game_id, &settings);

    let entity = server_world
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

    entity
}

/// Command to create a new game
///
/// Must be infallible
pub struct NewGameCommand {
    /// Settings to setup the new game
    pub new_game_settings: NewGameSettings,
    /// The new games id issued by the auth server
    pub new_game_id: GameId,
    /// The player who requested to start the game
    pub owning_player: Option<AccountId>,
}

impl Command for NewGameCommand {
    fn apply(self, server_world: &mut bevy::prelude::World) {
        let max_players = self.new_game_settings.max_player_count;
        let mut id_service = ObjectIdService::new();

        server_world.resource_scope(
            |_world: &mut World, channel: Mut<AsyncChannelSender<InsertGamesMetaRow>>| {
                let _ = channel.sender_channel.send(InsertGamesMetaRow {
                    game_id: self.new_game_id,
                    max_players,
                    object_id_service: id_service.clone(),
                    owning_player: self.owning_player,
                    game_name: self.new_game_settings.game_name.clone(),
                });
            },
        );
        server_world.resource_scope(
            |_world: &mut World, channel: Mut<AsyncChannelSender<CreateGameCurvesTable>>| {
                let _ = channel.sender_channel.send(CreateGameCurvesTable {
                    game_id: self.new_game_id,
                });
            },
        );
        server_world.resource_scope(
            |_world: &mut World, channel: Mut<AsyncChannelSender<CreateGamePlayersTable>>| {
                let _ = channel.sender_channel.send(CreateGamePlayersTable {
                    game_id: self.new_game_id,
                });
            },
        );

        let game_id = generate_new_game(
            server_world,
            self.new_game_settings,
            self.new_game_id,
            &mut id_service,
        );
        server_world.resource_scope(|_world: &mut World, mut mapping: Mut<GameIdMapping>| {
            mapping.map.insert(self.new_game_id, game_id)
        });
    }
}
