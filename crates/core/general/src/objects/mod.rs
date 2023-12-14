use bevy::ecs::system::Resource;
use serde::{Deserialize, Serialize};

use self::core_components::ObjectId;

pub mod core_components;
pub mod outpost;

/// A persistent service used to generate new unique ids. It is saved into the games meta db
#[derive(Resource, Serialize, Deserialize, Clone, Debug)]
pub struct ObjectIdService {
    max_id_next: u32,
    available_ids: Vec<u32>,
}

impl ObjectIdService {
    pub fn new() -> ObjectIdService {
        ObjectIdService {
            max_id_next: 10,
            available_ids: vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9],
        }
    }

    /// Gets the next available id and ensures that the available ids is at least full
    pub fn next_id(&mut self) -> u32 {
        let next_id = self.available_ids.pop().unwrap();
        while self.available_ids.len() < 10 {
            self.available_ids.push(self.max_id_next);
            self.max_id_next = self.max_id_next + 1;
        }
        next_id
    }

    /// Returns an id to the service, making it available to be used again
    pub fn return_id(&mut self, id: u32) {
        self.available_ids.push(id);
        self.available_ids.sort_by(|a, b| a.cmp(b));
    }

    /// Returns a new [`ObjectId`] with a valid id
    pub fn new_object_id(&mut self) -> ObjectId {
        ObjectId::new(self.next_id())
    }
}
