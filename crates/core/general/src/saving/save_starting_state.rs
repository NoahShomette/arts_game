use bevy::ecs::{
    system::{Query, SystemState},
    world::World,
};

use crate::{
    game_state::{ObjectState, ObjectsState},
    objects::core_components::ObjectId,
};

use super::SaveId;

/// Iterates through every component in the world that has an [`ObjectId`] and saves all of the components in it that implement [`SaveTrait`]
pub fn save_world(world: &mut World) -> ObjectsState {
    let mut objects_state = ObjectsState {
        objects_state: vec![],
    };

    let mut system_state: SystemState<Query<(&ObjectId, &dyn SaveId)>> = SystemState::new(world);

    let query = system_state.get_mut(world);

    for (object_id, saveable_components) in query.iter() {
        let mut state: (ObjectId, Vec<(u8, String)>) = (object_id.clone(), vec![]);

        for save in saveable_components.iter() {
            if let Some(info) = save.save() {
                state.1.push(info);
            }
        }

        objects_state.objects_state.push(ObjectState { state });
    }

    objects_state
}
