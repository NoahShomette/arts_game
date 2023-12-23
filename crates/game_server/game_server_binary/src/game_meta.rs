//! Handles meta game related things
//!

use bevy::ecs::{component::Component, system::Resource};
use core_library::auth_server::AccountId;

/// Resource inserted into the game world and component attached to the [`crate::game_manager::GameInstance`] entity that holds the player who "owns" the game
#[derive(Resource, Component)]
pub struct OwningPlayer {
    pub player_id: AccountId,
}
