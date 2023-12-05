use bevy::utils::Uuid;
use serde::{Deserialize, Serialize};

/// A wrapper for a player id assigned from the AuthServer
#[derive(Serialize, Deserialize, Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct PlayerId {
    pub id: Uuid,
}
