//! Responsible for handling the actual game connection with players.
//!
//! When players are connected, it will send any curve changes that they can see as well as accept player Actions to change the game state

use std::net::SocketAddr;

use bevy::{
    app::{Plugin, Update},
    ecs::{
        component::Component,
        event::{EventReader, EventWriter},
        schedule::{IntoSystemConfigs, OnEnter},
        system::{Res, ResMut, Resource},
    },
    log::{error, info},
    tasks::{TaskPool, TaskPoolBuilder},
    utils::HashMap,
};
use bevy_eventwork::{
    AppNetworkMessage, ConnectionId, EventworkRuntime, Network, NetworkData, NetworkEvent,
};
use bevy_eventwork_mod_websockets::{NetworkSettings, WebSocketProvider};
use core_library::{
    auth_server::AccountId,
    authentication::AppAuthenticationState,
    game_meta::GameId,
    network::{ws_game_server::ClientInitialConnect, GameAddrInfo},
};

use crate::{
    app::app_scheduling::ServerAuthenticatedSets,
    game_manager::client_game_connection::RemoveConnectedPlayerFromGameEvent,
};

pub struct GameServerPlugin;

impl Plugin for GameServerPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugins(bevy_eventwork::EventworkPlugin::<
            WebSocketProvider,
            bevy::tasks::TaskPool,
        >::default());

        app.listen_for_message::<ClientInitialConnect, WebSocketProvider>();

        app.init_resource::<ConnectionIdPlayerIdMapping>()
            .init_resource::<PlayerIdGameIdMapping>();

        app.insert_resource(NetworkSettings::default());
        app.insert_resource(EventworkRuntime(
            TaskPoolBuilder::new().num_threads(2).build(),
        ));

        app.add_systems(
            OnEnter(AppAuthenticationState::Authenticated),
            setup_networking,
        );

        app.add_systems(
            Update,
            (
                handle_connection_events,
                handle_client_initial_connection_events,
            )
                .in_set(ServerAuthenticatedSets::ServerTasks),
        );
    }
}

/// Maps connection_ids to their player ids
#[derive(Resource, Default)]
pub struct ConnectionIdPlayerIdMapping {
    pub map: HashMap<ConnectionId, Option<AccountId>>,
}

/// Maps player ids to a game id, if the player is in that game
#[derive(Resource, Default)]
pub struct PlayerIdGameIdMapping {
    pub map: HashMap<AccountId, Option<GameId>>,
}

/// Component inserted on a Game Entity that holds what players are currently connected to it if there are any
#[derive(Component)]
pub struct CurrentlyConnectedPlayers {
    pub players: Vec<AccountId>,
}

impl CurrentlyConnectedPlayers {
    /// Creates a new Connected Players
    #[allow(dead_code)]
    pub fn new() -> CurrentlyConnectedPlayers {
        CurrentlyConnectedPlayers { players: vec![] }
    }

    /// Creates a new Connected Players with the given id in it
    pub fn new_with_id(player_id: AccountId) -> CurrentlyConnectedPlayers {
        CurrentlyConnectedPlayers {
            players: vec![player_id],
        }
    }

    /// Inserts a Player Id into the list
    pub fn insert(&mut self, player_id: AccountId) {
        self.players.push(player_id)
    }

    /// Removes all instances of a player id from the list
    pub fn remove(&mut self, player_id: &AccountId) {
        self.players.retain(|x| x != player_id);
    }

    /// Checks if the given player id is present
    #[allow(dead_code)]
    pub fn contains(&self, player_id: &AccountId) -> bool {
        self.players.contains(player_id)
    }
}

fn setup_networking(
    mut net: ResMut<Network<WebSocketProvider>>,
    settings: Res<NetworkSettings>,
    task_pool: Res<EventworkRuntime<TaskPool>>,
    server_info: Res<GameAddrInfo>,
) {
    match net.listen(
        SocketAddr::new(
            server_info
                .server_addr
                .parse()
                .expect("Ip Address not valid"),
            server_info.ws_port,
        ),
        &task_pool.0,
        &settings,
    ) {
        Ok(_) => (),
        Err(err) => {
            error!("Could not start listening: {}", err);
            panic!();
        }
    }
    info!("Started listening for new connections!");
}

fn handle_connection_events(
    mut connection_id_mapping: ResMut<ConnectionIdPlayerIdMapping>,
    mut network_events: EventReader<NetworkEvent>,
    mut remove_players: EventWriter<RemoveConnectedPlayerFromGameEvent>,
) {
    for event in network_events.read() {
        match event {
            NetworkEvent::Connected(conn_id) => {
                connection_id_mapping.map.insert(*conn_id, None);
            }
            NetworkEvent::Disconnected(conn_id) => {
                // If we have a player id then we send an event to remove it from any games that it may or may not be in
                if let Some(Some(player_id)) = connection_id_mapping.map.get(conn_id) {
                    remove_players.send(RemoveConnectedPlayerFromGameEvent {
                        player_id: player_id.clone(),
                    })
                }
                connection_id_mapping.map.remove(conn_id);
            }
            NetworkEvent::Error(err) => info!("Error connecting Client: {}", err),
        }
    }
}

/// Processes all initial connection messages and updates the connection id to player id matches with the new player id
fn handle_client_initial_connection_events(
    mut connection_id_mapping: ResMut<ConnectionIdPlayerIdMapping>,
    mut network_events: EventReader<NetworkData<ClientInitialConnect>>,
) {
    for event in network_events.read() {
        connection_id_mapping
            .map
            .insert(*event.source(), Some(event.player_id.clone()));
    }
}
