//! Responsible for ticking game ticks
//!
//!

use bevy::{
    app::{FixedUpdate, Plugin},
    ecs::{
        entity::Entity,
        system::{Query, Resource, SystemState},
        world::{Mut, World},
    },
};

use crate::game_manager::{game_schedule::GameWorldSimulationSchedule, GameInstance};

pub struct GameRunnerPlugin;

impl Plugin for GameRunnerPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        let system_state: SystemState<Query<(Entity, &mut GameInstance)>> =
            SystemState::new(&mut app.world);
        app.insert_resource(CachedSystemState {
            games_query: system_state,
        });
        app.add_systems(FixedUpdate, tick_games);
    }
}

#[derive(Resource)]
struct CachedSystemState {
    games_query: SystemState<Query<'static, 'static, (Entity, &'static mut GameInstance)>>,
}

fn tick_games(world: &mut World) {
    world.resource_scope(|world, mut query: Mut<CachedSystemState>| {
        let mut games_query = query.games_query.get_mut(world);

        for (_entity, mut game) in games_query.iter_mut() {
            game.game_tick.game_tick += 1;
            // If the new tick - the simulation tick amount is greater than or equal to the last time the game was simulated,
            // we need to simulate it again
            if game
                .game_tick
                .game_tick
                .saturating_sub(game.game_tick.simulation_tick_amount)
                >= game.game_tick.last_simulated_tick
            {
                game.game_world.run_schedule(GameWorldSimulationSchedule);
                game.game_tick.last_simulated_tick = game.game_tick.game_tick
            }
        }
    });
}
