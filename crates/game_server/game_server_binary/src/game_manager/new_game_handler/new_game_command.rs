use std::sync::{
    mpsc::{self, Receiver, Sender},
    Arc, Mutex,
};

use bevy::ecs::system::{Command, Resource};
use core_library::{auth_server::game::GameId, game_meta::NewGameSettings};

use crate::game_manager::new_game::generate_new_game;

#[derive(Resource, Clone)]
pub struct NewGameCommandsChannel {
    pub sender_channel: Sender<NewGameCommand>,
    pub reciever_channel: Arc<Mutex<Receiver<NewGameCommand>>>,
}

impl NewGameCommandsChannel {
    pub fn new() -> Self {
        let (sender, reciever) = mpsc::channel::<NewGameCommand>();

        NewGameCommandsChannel {
            sender_channel: sender,
            reciever_channel: Arc::new(Mutex::new(reciever)),
        }
    }
}

/// Command to create a new game
pub struct NewGameCommand {
    pub new_game_settings: NewGameSettings,
    pub new_game_id: GameId,
}

impl Command for NewGameCommand {
    fn apply(self, world: &mut bevy::prelude::World) {
        generate_new_game(world, self.new_game_settings, self.new_game_id);
    }
}
