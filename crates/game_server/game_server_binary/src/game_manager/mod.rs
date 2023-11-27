//! Responsible for establishing and managing active games. Each game gets their own GameInstance
//! which holds all their state between the GameInstance and the stuff in the actual world.
//!
//! Is also responsible though game_manager::save_manager with saving each game into the server files

use arts_core::auth_server::game::GameId;
use bevy::{
    app::Plugin,
    ecs::{
        component::Component, entity::Entity, schedule::Schedule, system::Resource, world::World,
    },
    utils::HashMap,
};

use crate::player_actions::PlayerAction;

use self::{game_schedule::GameWorldSimulationSchedule, save_manager::SaveManagerPlugin};

pub mod game_schedule;
mod new_game_handler;
mod save_manager;

pub struct GameManagerPlugin;

impl Plugin for GameManagerPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_schedule(Schedule::new(GameWorldSimulationSchedule));
        app.add_plugins(SaveManagerPlugin);
    }
}

/// A resource that contains mappings from [`GameId`] -> [`Entity`]. This allows easy access to any specific game
#[derive(Resource)]
pub struct GameIdMapping {
    pub map: HashMap<GameId, Entity>,
}

/// An instance of a game
#[derive(Component)]
pub struct GameInstance {
    /// The id given to the game by the server
    pub game_id: GameId,
    /// The world that contains the game state
    pub game_world: World,
    /// All actions queued by players for the future. Is kept in sync with the save file manager
    pub future_actions: Vec<PlayerAction>,
    /// current tick and information used to tick the game
    pub game_tick: GameTickInfo,
}

pub struct GameTickInfo {
    /// The current game tick
    pub game_tick: u64,
    /// The amount of ticks that will be ticked every time the server is ticked.
    ///
    /// These are ticked sequentually with the fixed time step running in full for every tick
    pub ticks_per_tick: u64,
    /// The minimum amount of ticks before the game will simulate.
    ///
    /// Every this many ticks, the game will execute and actually process everything
    /// that occured between the current tick and the last time this game was simulated
    pub simulation_tick_amount: u64,
    /// Holds the last tick that this game was simulated
    pub last_simulated_tick: u64,
}
