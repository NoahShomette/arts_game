use bevy::{ecs::system::Resource, tasks::TaskPool};

pub mod actions;
pub mod async_runners;
pub mod auth_server;
pub mod authentication;
pub mod game_meta;
pub mod network;
pub mod objects;
pub mod player;

#[derive(Resource)]
pub struct TaskPoolRes(pub TaskPool);
