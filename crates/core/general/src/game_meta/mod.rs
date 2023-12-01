//! Responsible for meta information on games like settings and the like

use serde::{Deserialize, Serialize};

pub struct GameSettings;

/// Settings needed to construct a new game
#[derive(Serialize, Deserialize)]
pub struct NewGameSettings {
    pub player_count: u8,
    pub map_size: MapSize,
    pub connection_density: ConnectionDensity,
    pub ticks_per_tick: u64,
    pub simulation_tick_amount: u64,
}

#[derive(Serialize, Deserialize)]
pub enum MapSize {
    Small,
    Medium,
    Large,
    Custom { outpost_count: u16 },
}

impl MapSize {
    pub fn outpost_count(&self) -> &u16 {
        match self {
            MapSize::Small => &35,
            MapSize::Medium => &55,
            MapSize::Large => &75,
            MapSize::Custom { outpost_count } => outpost_count,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub enum ConnectionDensity {
    Dense,
    Sparse,
}
