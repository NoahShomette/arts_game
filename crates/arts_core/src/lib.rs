use bevy::{ecs::system::Resource, tasks::TaskPool};

pub mod actions;
pub mod async_runners;
pub mod authentication;
pub mod game;
#[cfg(feature = "http_server")]
pub mod http_server;
pub mod network;
pub mod user_data;

#[derive(Resource)]
pub struct TaskPoolRes(pub TaskPool);
