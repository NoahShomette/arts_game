//! Module managing client level player information and things

use bevy::ecs::system::Resource;

#[derive(Resource)]
pub struct PlayerInformation {
    pub username: String,
}
