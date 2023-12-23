use bevy::{ecs::world::World, math::Vec2};
use bevy_state_curves::prelude::{CurveTrait, SteppedCurve};
use general::{
    game_meta::{GameId, NewGameSettings},
    game_simulation::GameWorldSimulationSchedule,
    objects::{
        core_components::{ObjectId, ObjectPosition},
        ObjectIdService,
    },
    AsyncChannelSender,
};

use sqlite_database::{
    game_world_setup_saving, saving::SaveSchedule,
    schemes::game_server::game_tables::InsertGameCurvesRow,
};

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
    game_world.add_schedule(SaveSchedule::new_schedule());
    game_world.add_schedule(GameWorldSimulationSchedule::new_schedule());
    game_world_setup_saving(server_world, &mut game_world);
    game_world
}

pub fn insert_new_game_state(
    game_world: &mut World,
    new_game_id: &GameId,
    settings: &NewGameSettings,
) {
    let insert_game_curves_row = game_world
        .remove_resource::<AsyncChannelSender<InsertGameCurvesRow>>()
        .expect("Update Row resource should be in main world");

    let mut id_service = game_world.remove_resource::<ObjectIdService>().expect(
        "ObjectIdService must be inserted into game world prior to updating any game_world_state",
    );

    for i in 0..*settings.map_point_count.map_point_count() {
        let mut pos = SteppedCurve::<ObjectPosition>::new();

        pos.insert_keyframe(
            0,
            ObjectPosition {
                position: Vec2::new(i as f32, i as f32),
            },
        );

        let id = id_service.new_object_id();
        let Some(row) = InsertGameCurvesRow::new_row(*new_game_id, &id, None, &pos) else {
            continue;
        };
        let _ = insert_game_curves_row.sender_channel.send(row);

        game_world.spawn((pos, ObjectId::new(id.id)));
    }

    game_world.insert_resource(id_service);
    game_world.insert_resource(insert_game_curves_row);
}
