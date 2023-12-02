use bevy::{ecs::system::Resource, tasks::TaskPool};

pub mod actions;
pub mod async_runners;
pub mod auth_server;
pub mod authentication;
pub mod game_meta;
pub mod map;
pub mod network;

#[derive(Resource)]
pub struct TaskPoolRes(pub TaskPool);
