use bevy_state_curves::GameTick;
use serde::{Deserialize, Serialize};

use crate::{auth_server::AccountId, objects::core_components::ObjectId};

pub struct GameState {
    pub starting_state: ObjectsState,
    pub actions: GameActions,
    pub patches: GameStatePatches,
}

/// The state of the game objects in the game
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct ObjectsState {
    pub objects_state: Vec<ObjectState>,
}

/// A list of actions in the game
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct GameActions {
    pub actions: Vec<ActionInfo>,
}

/// A list of patches
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct GameStatePatches {
    pub patches: Vec<PatchInfo>,
}

/// A singular objects state in the game
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ObjectState {
    pub state: (ObjectId, Vec<(u8, String)>),
}

/// An action and meta information on it
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ActionInfo {}

/// A patch for game state and meta information on how to use it
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PatchInfo {
    pub tick_patch_is_valid: GameTick,
    pub patch: ObjectState,
    pub players_to_receive_patch: Vec<AccountId>,
}
