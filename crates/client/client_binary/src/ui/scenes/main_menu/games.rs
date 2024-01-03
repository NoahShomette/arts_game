//! The games menu does three things
//! - Show a player all games they are in
//! - Show a player all public open games they can join
//! - Create a new game
//!
//! The first two options should basically be tabs

use bevy::app::Plugin;

pub struct GamesPlugin;

impl Plugin for GamesPlugin {
    fn build(&self, _app: &mut bevy::prelude::App) {}
}
