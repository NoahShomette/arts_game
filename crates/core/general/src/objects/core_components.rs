/// Core components every object will have
use bevy::{ecs::component::Component, math::Vec2};
use bevy_state_curves::prelude::SteppedKeyframe;
use serde::{Deserialize, Serialize};

use crate::auth_server::AccountId;

/// An Id uniquely identifying an object in the game state
#[derive(Component, Serialize, Deserialize, Debug)]
pub struct ObjectId {
    pub id: u32,
}

impl ObjectId {
    pub fn new(id: u32) -> ObjectId {
        Self { id }
    }
}

/// The Position of an object
#[derive(Clone, Component, Serialize, Deserialize, Debug)]
pub struct ObjectPosition {
    pub position: Vec2,
}

impl SteppedKeyframe<ObjectPosition> for ObjectPosition {}

/// Component that holds what General (Player basically) controls this unit
#[derive(Clone, Component, Serialize, Deserialize, Debug, Default)]
pub struct ObjectGeneral {
    id: Option<AccountId>,
}

impl SteppedKeyframe<ObjectGeneral> for ObjectGeneral {}
