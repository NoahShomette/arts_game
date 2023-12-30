use serde::{Deserialize, Serialize};

pub const MAX_USERNAME_LENGTH: usize = 24;

/// A request to set a players username
#[derive(Serialize, Deserialize, Clone)]
pub struct SetPlayerUsernameRequest {
    pub username: String,
}

/// A response from [`SetPlayerUsernameRequest`] that contains the new username. Returned when successful
#[derive(Serialize, Deserialize, Clone)]
pub enum SetPlayerUsernameResponse {
    Ok { new_username: String },
    Error { error_text: String },
}

/// A response to requests for a players username. Contains the new username
#[derive(Serialize, Deserialize, Clone)]
pub struct GetPlayerUsernameResponse {
    pub username: String,
}
