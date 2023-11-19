use bevy::{ecs::system::Resource, tasks::TaskPool};

pub mod actions;
pub mod async_runners;
pub mod auth_server;
pub mod authentication;
#[cfg(feature = "http_server")]
pub mod http_server;
pub mod network;

#[derive(Resource)]
pub struct TaskPoolRes(pub TaskPool);
