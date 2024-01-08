use bevy::{
    ecs::{component::Component, world::World},
    math::Vec2,
};
use bevy_state_curves::prelude::{CurveTrait, SteppedCurve};
use general::{
    actions::PlayerAction,
    game_meta::{GameId, NewGameSettings},
    game_simulation::GameWorldSimulationSchedule,
    objects::{
        core_components::{ObjectId, ObjectPosition},
        ObjectIdService,
    },
};

use sqlite_database::game_world_setup_db;

pub mod new_game;

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

/// Function responsible for generating the new game world with the correct default setup. This is used for both loading old games and starting new games
pub fn create_game_world(
    server_world: &mut World,
    game_id: &GameId,
    _settings: &NewGameSettings,
    id_service: &mut ObjectIdService,
) -> World {
    let mut game_world = World::new();
    game_world.insert_resource(id_service.clone());
    game_world.insert_resource(*game_id);
    game_world.add_schedule(GameWorldSimulationSchedule::new_schedule());
    game_world_setup_db(server_world, &mut game_world);
    game_world
}

pub fn insert_new_game_state(game_world: &mut World, settings: &NewGameSettings) {
    let mut id_service = game_world.remove_resource::<ObjectIdService>().expect(
        "ObjectIdService must be inserted into game world prior to updating any game_world_state",
    );

    for i in 0..*settings.map_point_count.map_point_count() {
        let mut pos: SteppedCurve<ObjectPosition> = SteppedCurve::<ObjectPosition>::new();

        pos.insert_keyframe(
            0,
            ObjectPosition {
                position: Vec2::new(i as f32, i as f32),
            },
        );

        let id = id_service.new_object_id();

        game_world.spawn((pos, ObjectId::new(id.id)));
    }

    game_world.insert_resource(id_service);
}
