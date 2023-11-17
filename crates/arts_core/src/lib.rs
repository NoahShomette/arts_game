use bevy::{ecs::system::Resource, tasks::TaskPool};

pub mod async_runners;
pub mod authentication;
pub mod game;
pub mod network;
pub mod actions;

#[derive(Resource)]
pub struct TaskPoolRes(pub TaskPool);
