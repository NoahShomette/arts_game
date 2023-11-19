use bevy::utils::Uuid;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, PartialOrd, Ord, Eq)]
pub struct GameId {
    pub id: Uuid,
}
