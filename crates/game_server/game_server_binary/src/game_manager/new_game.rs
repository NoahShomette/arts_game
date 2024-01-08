//! System that generates a new game

use bevy::{
    app::{Plugin, Update},
    ecs::{
        schedule::IntoSystemConfigs,
        system::{Commands, ResMut},
        world::{Mut, World},
    },
};
use core_library::{
    auth_server::AccountId, game_management::new_game::generate_new_game, game_meta::GameId,
    game_meta::NewGameSettings, objects::ObjectIdService,
    sqlite_database::schemes::game_server::games::InsertGamesTableRow, AsyncChannelSender,
};

use bevy::ecs::system::Command;

use crate::app::app_scheduling::ServerAuthenticatedSets;

use super::{new_game_http::requests::NewGameCommandsChannel, GameIdMapping};

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

        let (game_instance_entity, starting_game_state) = generate_new_game(
            server_world,
            &self.new_game_settings,
            &self.new_game_id,
            &mut id_service,
        );

        server_world.resource_scope(
            |_world: &mut World, channel: Mut<AsyncChannelSender<InsertGamesTableRow>>| {
                let _ = channel.sender_channel.send(InsertGamesTableRow {
                    game_id: self.new_game_id,
                    max_players,
                    object_id_service: id_service.clone(),
                    owning_player: self.owning_player,
                    game_name: self.new_game_settings.game_name.clone(),
                    starting_state: starting_game_state.starting_state,
                    game_actions: starting_game_state.actions,
                    game_state_patches: starting_game_state.patches,
                });
            },
        );

        server_world.resource_scope(|_world: &mut World, mut mapping: Mut<GameIdMapping>| {
            mapping.map.insert(self.new_game_id, game_instance_entity)
        });
    }
}
