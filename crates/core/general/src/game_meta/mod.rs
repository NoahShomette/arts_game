//! Responsible for meta information on games like settings and the like

use bevy::{
    ecs::{component::Component, system::Resource},
    math::Vec2,
    utils::Uuid,
};
use serde::{Deserialize, Serialize};

use crate::auth_server::AccountId;

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, PartialOrd, Ord, Eq, Hash)]
pub struct GameId {
    pub id: Uuid,
}

/// Meta settings on a game
pub struct GameSettings {
    pub max_player_count: u8,
    pub map_size: MapSize,
    pub connection_density: ConnectionDensity,
}

/// Settings that can be changed and must be supplied when starting a new game
#[derive(Serialize, Deserialize)]
pub struct NewGameSettings {
    pub max_player_count: u8,
    pub outpost_count: OutpostCount,
    pub map_size: MapSize,
    pub connection_density: ConnectionDensity,
    pub ticks_per_tick: u64,
    pub simulation_tick_amount: u64,
}

/// The map dimensions. Representing the total physical size of the map
#[derive(Serialize, Deserialize)]
pub enum MapSize {
    Small,
    Medium,
    Large,
    Custom { dimensions: Vec2 },
}

impl MapSize {
    pub fn map_size(&self) -> Vec2 {
        match self {
            MapSize::Small => Vec2::new(200.0, 200.0),
            MapSize::Medium => Vec2::new(300.0, 300.0),
            MapSize::Large => Vec2::new(400.0, 400.0),
            MapSize::Custom { dimensions } => dimensions.clone(),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub enum ConnectionDensity {
    Dense,
    Sparse,
}

/// The amount of outposts on a map
#[derive(Serialize, Deserialize)]
pub enum OutpostCount {
    Small,
    Medium,
    Large,
    Custom { outpost_count: u16 },
}

impl OutpostCount {
    pub fn outpost_count(&self) -> &u16 {
        match self {
            OutpostCount::Small => &35,
            OutpostCount::Medium => &55,
            OutpostCount::Large => &75,
            OutpostCount::Custom { outpost_count } => outpost_count,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Component)]
pub struct GamePlayers {
    pub players: Vec<AccountId>,
}

impl GamePlayers {
    /// Creates a new Connected Players
    pub fn new() -> GamePlayers {
        GamePlayers { players: vec![] }
    }

    /// Creates a new Connected Players with the given id in it
    pub fn new_with_id(player_id: AccountId) -> GamePlayers {
        GamePlayers {
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
    pub fn contains(&self, player_id: &AccountId) -> bool {
        self.players.contains(player_id)
    }
}
