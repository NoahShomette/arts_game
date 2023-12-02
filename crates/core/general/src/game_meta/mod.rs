//! Responsible for meta information on games like settings and the like

use bevy::math::Vec2;
use serde::{Deserialize, Serialize};

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
