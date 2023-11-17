//! Responsible for ticking game ticks
//!
//!

use bevy::{
    app::{FixedUpdate, Plugin},
    ecs::{
        entity::Entity,
        system::{Commands, Query},
    },
};

use crate::game_manager::{GameInstance, GameNeedsSimulating};

pub struct GameRunnerPlugin;

impl Plugin for GameRunnerPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(FixedUpdate, tick_games);
    }
}

fn tick_games(mut games: Query<(Entity, &mut GameInstance)>, mut commands: Commands) {
    for (entity, mut game) in games.iter_mut() {
        game.game_tick.game_tick += game.game_tick.ticks_per_tick;
        // If the new tick - the simulation tick amount is greater than or equal to the last time the game was simulated,
        // we need to simulate it again
        if game
            .game_tick
            .game_tick
            .saturating_sub(game.game_tick.simulation_tick_amount)
            >= game.game_tick.last_simulated_tick
        {
            commands.entity(entity).insert(GameNeedsSimulating);
        }
    }
}
