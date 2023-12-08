use std::sync::{
    mpsc::{self, Receiver, Sender},
    Arc, Mutex,
};

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

/// A generic channel used to commmunicate in and out of threads
#[derive(Resource, Clone)]
pub struct AsyncChannel<T> {
    pub sender_channel: Sender<T>,
    pub reciever_channel: Arc<Mutex<Receiver<T>>>,
}

impl<T> AsyncChannel<T> {
    pub fn new() -> Self {
        let (sender, reciever) = mpsc::channel::<T>();

        AsyncChannel {
            sender_channel: sender,
            reciever_channel: Arc::new(Mutex::new(reciever)),
        }
    }
}

impl<T> Default for AsyncChannel<T> {
    fn default() -> Self {
        Self::new()
    }
}
