use std::sync::{
    mpsc::{self, Receiver, Sender},
    Arc, Mutex,
};

use bevy::{
    ecs::{component::Component, system::Resource, world::World},
    tasks::TaskPool,
};

pub mod actions;
pub mod async_runners;
pub mod auth_server;
pub mod authentication;
pub mod game_meta;
pub mod game_simulation;
pub mod network;
pub mod objects;
pub mod player;

#[derive(Resource)]
pub struct TaskPoolRes(pub TaskPool);

/// Receiver half of an [`mpsc`] Channel
///
/// For game world related purposes this is kept in the main world and receives messages
///  from games and processes all games messages of the same type at the same time.
#[derive(Resource, Clone)]
pub struct AsyncChannelReceiver<T> {
    pub reciever_channel: Arc<Mutex<Receiver<T>>>,
}

/// Sender half of an [`mpsc`] Channel
///
/// For game world related purposes this is cloned to each game world and is used to queue commands
#[derive(Resource, Clone)]
pub struct AsyncChannelSender<T> {
    pub sender_channel: Sender<T>,
}

/// Clones the [`AsyncChannelReceiver`] of the given type if it exists in the world. Should be used on the server_world
pub fn clone_async_sender<T: 'static + Send + Sync + Clone>(
    server_world: &World,
) -> Option<AsyncChannelSender<T>> {
    server_world
        .get_resource::<AsyncChannelSender<T>>()
        .cloned()
}

/// Creates an [`AsyncChannelSender`] and an [`AsyncChannelReceiver`] for the given type
pub fn create_async_channel<T>() -> (AsyncChannelSender<T>, AsyncChannelReceiver<T>) {
    let (sender, reciever) = mpsc::channel::<T>();
    (
        AsyncChannelSender {
            sender_channel: sender,
        },
        AsyncChannelReceiver {
            reciever_channel: Arc::new(Mutex::new(reciever)),
        },
    )
}

/// A Component that represents data that must be saved into the database and *DOES NOT* normally exist for multiple frames
#[derive(Component)]
pub struct PendingDatabaseData<T> {
    pub data: T,
}

/// An MPSC async channel. Use this for single use communications that only need to be used in a single world.
///
/// Use the [`AsyncChannelSender`]/[`AsyncChannelReceiver`] duo if multiple worlds will need to access the channel
#[derive(Resource, Clone)]
pub struct AsyncChannel<T> {
    pub sender_channel: Sender<T>,
    pub reciever_channel: Arc<Mutex<Receiver<T>>>,
}

impl<T> Default for AsyncChannel<T> {
    fn default() -> Self {
        let (sender, reciever) = mpsc::channel::<T>();

        Self {
            sender_channel: sender,
            reciever_channel: Arc::new(Mutex::new(reciever)),
        }
    }
}
