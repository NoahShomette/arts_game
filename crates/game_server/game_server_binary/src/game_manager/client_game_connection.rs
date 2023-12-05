use bevy::{
    app::{Plugin, Update},
    ecs::{
        entity::Entity,
        event::{Event, EventReader, EventWriter},
        schedule::IntoSystemConfigs,
        system::{Commands, Query, Res, ResMut},
    },
};
use bevy_eventwork::{AppNetworkMessage, NetworkData};
use bevy_eventwork_mod_websockets::WebSocketProvider;
use core_library::{
    authentication::AuthenticationServerInfo, game_meta::GameId,
    network::ws_game_server::ClientConnectToGame, player::AccountId,
};

use crate::{
    app_authentication::auth_user_request_blocking,
    client_game_server_network::{ConnectedPlayers, PlayerIdGameIdMapping},
};

use super::GameIdMapping;

pub struct ClientGameConnectionPlugin;

impl Plugin for ClientGameConnectionPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.listen_for_message::<ClientConnectToGame, WebSocketProvider>();
        app.add_event::<AddConnectedPlayerToGameEvent>();
        app.add_event::<RemoveConnectedPlayerFromGameEvent>();

        app.add_systems(
            Update,
            (
                handle_connecting_to_games,
                (
                    add_connected_player_to_game,
                    remove_connected_player_from_game,
                ),
            )
                .chain(),
        );
    }
}

#[derive(Event)]
pub struct AddConnectedPlayerToGameEvent {
    pub game_id: GameId,
    pub player_id: AccountId,
}

#[derive(Event)]
pub struct RemoveConnectedPlayerFromGameEvent {
    pub player_id: AccountId,
}

/// Maps a connection_id and player id to a specific game.
///
/// Removes that player from any previous game connections if they were in any
pub fn handle_connecting_to_games(
    mut new_messages: EventReader<NetworkData<ClientConnectToGame>>,
    mut event_writer: EventWriter<AddConnectedPlayerToGameEvent>,
    auth_server: Res<AuthenticationServerInfo>,
) {
    for message in new_messages.read() {
        // Verify that this is a valid user requesting to connect to the game
        let Ok(_) =
            auth_user_request_blocking(message.access_token.clone(), auth_server.addr.clone())
        else {
            continue;
        };

        event_writer.send(AddConnectedPlayerToGameEvent {
            game_id: message.game_id.clone(),
            player_id: message.player_id.clone(),
        });
    }
}

fn add_connected_player_to_game(
    mut new_messages: EventReader<AddConnectedPlayerToGameEvent>,
    game_id_mapping: Res<GameIdMapping>,
    mut player_game_id_mapping: ResMut<PlayerIdGameIdMapping>,
    mut games: Query<(Entity, Option<&mut ConnectedPlayers>)>,
    mut commands: Commands,
) {
    for message in new_messages.read() {
        // Get the games entity
        if let Some(game_entity) = game_id_mapping.map.get(&message.game_id) {
            // Get the game components
            if let Ok((entity, players)) = games.get_mut(*game_entity) {
                if let Some(mut players) = players {
                    // insert the player into the games connected players
                    players.insert(message.player_id.clone());
                } else {
                    commands
                        .entity(entity)
                        .insert(ConnectedPlayers::new_with_id(message.player_id.clone()));
                }
                // update the players id to game id mapping
                player_game_id_mapping
                    .map
                    .insert(message.player_id.clone(), Some(message.game_id.clone()));
            }
        }
    }
}

fn remove_connected_player_from_game(
    mut new_messages: EventReader<RemoveConnectedPlayerFromGameEvent>,
    game_id_mapping: Res<GameIdMapping>,
    mut player_game_id_mapping: ResMut<PlayerIdGameIdMapping>,
    mut games: Query<Option<&mut ConnectedPlayers>>,
) {
    for message in new_messages.read() {
        // If the player is in a game then we remove the player from that games connected players, otherwise we do nothing
        if let Some(game_id) = player_game_id_mapping.map.get(&message.player_id) {
            if let Some(game_id) = game_id {
                // Get the games entity
                if let Some(game_entity) = game_id_mapping.map.get(game_id) {
                    // Get the game components
                    if let Ok(players) = games.get_mut(*game_entity) {
                        if let Some(mut players) = players {
                            // insert the player into the games connected players
                            players.remove(&message.player_id);
                        }
                    }
                }

                // remove the PlayerId -> GameId mapping for the player
                player_game_id_mapping
                    .map
                    .insert(message.player_id.clone(), None);
            }
        }
    }
}
