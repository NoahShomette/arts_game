use std::sync::{
    mpsc::{self, Receiver, Sender},
    Arc, Mutex,
};

use bevy::prelude::Resource;
use ehttp::Response;

#[derive(Resource, Clone)]
pub struct AuthClient {
    pub sender_channel: Sender<Result<(ClientResponses, Response), String>>,
    pub reciever_channel: Arc<Mutex<Receiver<Result<(ClientResponses, Response), String>>>>,
}

impl AuthClient {
    pub fn new() -> Self {
        let (sender, reciever) = mpsc::channel::<Result<(ClientResponses, Response), String>>();

        AuthClient {
            sender_channel: sender,
            reciever_channel: Arc::new(Mutex::new(reciever)),
        }
    }
}

pub enum ClientResponses{
    SignIn,
    LogOut,
}
