//! Responsible for meta information on games like settings and the like

use bevy::{
    ecs::{component::Component, system::Resource},
    math::Vec2,
    reflect::TypePath,
    utils::Uuid,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::auth_server::AccountId;

#[derive(
    Serialize,
    Deserialize,
    Clone,
    Copy,
    PartialEq,
    PartialOrd,
    Ord,
    Eq,
    Hash,
    TypePath,
    Debug,
    Resource,
    Component,
)]
pub struct GameId {
    pub id: Uuid,
}

impl GameId {
    /// Converts self into a json string using serde
    pub fn to_json(self) -> String {
        serde_json::to_string(&self).unwrap()
    }

    /// Returns the interior Uuid as a String. Should only be used on occasions where you only need the [`Uuid`] itself and not the entire [`GameId`] object
    pub fn id_as_string(&self) -> String {
        self.id.to_string()
    }
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
    pub game_name: String,
    pub max_player_count: u8,
    pub map_point_count: MapPointCount,
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
            MapSize::Custom { dimensions } => *dimensions,
        }
    }
}

/// How many connections will be drawn between outposts
#[derive(Serialize, Deserialize)]
pub enum ConnectionDensity {
    Dense,
    Sparse,
}

/// The amount of points on a map
#[derive(Serialize, Deserialize)]
pub enum MapPointCount {
    Light,
    Normal,
    Dense,
    Custom { outpost_count: u16 },
}

impl MapPointCount {
    pub fn map_point_count(&self) -> &u16 {
        match self {
            MapPointCount::Light => &35,
            MapPointCount::Normal => &55,
            MapPointCount::Dense => &75,
            MapPointCount::Custom { outpost_count } => outpost_count,
        }
    }
}

/// Holds the [`AccountId`]s of every player that is actually playing in the game
#[derive(Serialize, Deserialize, Clone, Component, Default)]
pub struct GamePlayers {
    pub players: Vec<PlayerInformation>,
}

impl GamePlayers {
    /// Creates a new Connected Players with the given id in it
    pub fn new_with_id(player_id: AccountId) -> GamePlayers {
        GamePlayers {
            players: vec![PlayerInformation::new(player_id)],
        }
    }

    /// Inserts a Player Id into the list
    pub fn insert(&mut self, player_id: AccountId) {
        self.players.push(PlayerInformation::new(player_id))
    }

    /// Removes all instances of a player id from the list
    pub fn remove(&mut self, player_id: &AccountId) {
        self.players.retain(|x| &x.account_id != player_id);
    }

    /// Checks if the given player id is present
    pub fn contains(&self, player_id: &AccountId) -> bool {
        self.players
            .iter()
            .any(|info| &info.account_id == player_id)
    }

    /// Returns the amount of players in the game
    pub fn count(&self) -> u8 {
        self.players.len() as u8
    }
}

#[derive(Serialize, Deserialize, Clone, Component)]
pub struct PlayerInformation {
    account_id: AccountId,
    last_login_time: Option<DateTime<Utc>>,
    last_log_off_time: DateTime<Utc>,
    // The below are commented out as they are not yet implemented
    // faction: Faction,
    // player_relationships: Vec<(Relationship)>
}

impl PlayerInformation {
    pub fn new(player_id: AccountId) -> PlayerInformation {
        PlayerInformation {
            account_id: player_id,
            last_login_time: None,
            last_log_off_time: Utc::now(),
        }
    }
}
