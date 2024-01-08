use bevy::{
    app::App,
    ecs::{
        component::Component,
        entity::Entity,
        system::{Commands, Resource},
    },
    utils::HashMap,
};
use serde::{de::DeserializeOwned, Serialize};

pub mod save_starting_state;

pub trait SavingAppExtension {
    /// Registers a component for serialization and deserialization functions
    fn register_component<C>(&mut self)
    where
        C: Component + Serialize + DeserializeOwned + SaveId;

    /// Registers a resource for serialization and deserialization functions
    fn register_resource<R>(&mut self)
    where
        R: Resource + Serialize + DeserializeOwned + SaveId;
}

impl SavingAppExtension for App {
    fn register_component<C>(&mut self)
    where
        C: Component + Serialize + DeserializeOwned + SaveId,
    {
        let mut map = self.world.resource_mut::<SavingMap>();
        map.component_de_map
            .insert(C::save_id_const(), component_deserialize_onto::<C>);
    }

    fn register_resource<R>(&mut self)
    where
        R: Resource + Serialize + DeserializeOwned + SaveId,
    {
        let mut map = self.world.resource_mut::<SavingMap>();
        map.resource_de_map
            .insert(R::save_id_const(), resource_deserialize_onto::<R>);
    }
}

/// A resource that maps u8s to serialization and deserialization functions for components and resources
#[derive(Resource)]
pub struct SavingMap {
    pub component_de_map: HashMap<u8, ComponentDeserializeFn>,
    pub resource_de_map: HashMap<u8, ResourceDeserializeFn>,
}

pub type ComponentDeserializeFn = fn(data: &String, entity: Entity, commands: &mut Commands);

/// Deserializes a String component onto the given entity
pub fn component_deserialize_onto<T>(data: &String, entity: Entity, commands: &mut Commands)
where
    T: Serialize + DeserializeOwned + Component,
{
    let t = serde_json::from_str::<T>(data)
        .unwrap_or_else(|_| panic!("Failed to deserialize: {}", data));
    commands.entity(entity).insert(t);
}

pub type ResourceDeserializeFn = fn(data: &String, commands: &mut Commands);

/// Deserializes a String resource into the world given by the commands
pub fn resource_deserialize_onto<T>(data: &String, commands: &mut Commands)
where
    T: Serialize + DeserializeOwned + Resource,
{
    let t = serde_json::from_str::<T>(data)
        .unwrap_or_else(|_| panic!("Failed to deserialize: {}", data));
    commands.insert_resource(t);
}

/// Must be implemented on any components for objects that are expected to be saved
///
/// You must ensure that both this traits [save_id] function and [save_id_const] functions match
#[bevy_trait_query::queryable]
pub trait SaveId {
    fn save_id(&self) -> u8;
    fn save_id_const() -> u8
    where
        Self: Sized;

    /// Serializes self into a string
    fn to_string(&self) -> Option<String>;

    /// Saves self according to the implementation given in to_string
    fn save(&self) -> Option<(u8, String)> {
        let Some(data) = self.to_string() else {
            return None;
        };
        Some((self.save_id(), data))
    }
}
