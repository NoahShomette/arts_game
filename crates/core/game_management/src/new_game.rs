use bevy::ecs::{entity::Entity, world::World};
use general::{
    game_meta::{GameId, NewGameSettings},
    game_state::{GameActions, GameState, GameStatePatches},
    objects::ObjectIdService,
    saving::save_starting_state::save_world,
};

use crate::{create_game_world, insert_new_game_state, GameInstance};

/// Fn which generates a new game world, spawning it, and setting its initial state to the correct set and then returns the id and the initial starting state of the game world's entity
pub fn generate_new_game(
    server_world: &mut World,
    settings: &NewGameSettings,
    new_game_id: &GameId,
    id_service: &mut ObjectIdService,
) -> (Entity, GameState) {
    let mut game_world = create_game_world(server_world, new_game_id, settings, id_service);
    insert_new_game_state(&mut game_world, settings);
    let state: general::game_state::ObjectsState = save_world(&mut game_world);

    let entity = server_world
        .spawn(GameInstance {
            game_id: *new_game_id,
            game_world,
            future_actions: vec![],
            game_tick: super::GameTickInfo {
                game_tick: 0,
                ticks_per_tick: settings.ticks_per_tick,
                simulation_tick_amount: settings.simulation_tick_amount,
                last_simulated_tick: 0,
            },
        })
        .id();

    (
        entity,
        GameState {
            starting_state: state,
            actions: GameActions::default(),
            patches: GameStatePatches::default(),
        },
    )
}
