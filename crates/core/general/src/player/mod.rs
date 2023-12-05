use bevy::utils::Uuid;
use serde::{Deserialize, Serialize};

/// A wrapper for an id assigned from the AuthServer
#[derive(Serialize, Deserialize, Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct AccountId {
    pub id: Uuid,
}
