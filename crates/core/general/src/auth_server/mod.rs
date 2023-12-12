use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub mod game;
pub mod player_data;

/// A wrapper for an id assigned from the AuthServer
#[derive(Serialize, Deserialize, Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct AccountId {
    pub id: Uuid,
}
