//! System that generates a new game

use bevy::ecs::{entity::Entity, world::{World, Mut}};
use core_library::{
    game_generation::create_game_world, game_meta::GameId, game_meta::NewGameSettings,
};

use bevy::ecs::system::Command;

use super::{game_database::DatabaseConnection, GameInstance};

/// Fn which generates a new game world, spawning it, and setting its initial state to the correct set and then returns the id of the game world's entity
pub fn generate_new_game(
    main_world: &mut World,
    settings: NewGameSettings,
    new_game_id: GameId,
) -> Entity {
    let game_world = create_game_world(&settings);
    main_world
        .spawn(GameInstance {
            game_id: new_game_id,
            game_world,
            future_actions: vec![],
            game_tick: super::GameTickInfo {
                game_tick: 0,
                ticks_per_tick: settings.ticks_per_tick,
                simulation_tick_amount: settings.simulation_tick_amount,
                last_simulated_tick: 0,
            },
        })
        .id()
}

/// Command to create a new game
///
/// Must be infallible
pub struct NewGameCommand {
    pub new_game_settings: NewGameSettings,
    pub new_game_id: GameId,
}

impl Command for NewGameCommand {
    fn apply(self, world: &mut bevy::prelude::World) {
        generate_new_game(world, self.new_game_settings, self.new_game_id);

        world.resource_scope(|_world: &mut World, database: Mut<DatabaseConnection>| {

        });
    }
}
