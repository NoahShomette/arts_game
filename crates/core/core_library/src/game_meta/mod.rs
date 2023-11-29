//! Responsible for meta information on games like settings and the like

use serde::{Deserialize, Serialize};

pub struct GameSettings;

#[derive(Serialize, Deserialize)]
pub struct NewGameSettings {}
