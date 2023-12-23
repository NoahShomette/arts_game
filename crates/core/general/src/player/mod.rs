use serde::{Deserialize, Serialize};

pub const MAX_USERNAME_LENGTH: usize = 24;

#[derive(Serialize, Deserialize, Clone)]
pub struct SetPlayerUsernameRequest {
    pub username: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct SetPlayerUsernameResponse {
    pub new_username: String,
}
