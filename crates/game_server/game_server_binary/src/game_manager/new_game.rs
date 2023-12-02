//! System that generates a new game

use bevy::ecs::world::World;
use core_library::{
    auth_server::game::GameId, game_generation::create_game_world, game_meta::NewGameSettings,
};

use super::GameInstance;

pub fn generate_new_game(world: &mut World, settings: NewGameSettings, new_game_id: GameId) {
    let game_world = create_game_world(&settings);
    world.spawn(GameInstance {
        game_id: new_game_id,
        game_world,
        future_actions: vec![],
        game_tick: super::GameTickInfo {
            game_tick: 0,
            ticks_per_tick: settings.ticks_per_tick,
            simulation_tick_amount: settings.simulation_tick_amount,
            last_simulated_tick: 0,
        },
    });
}
