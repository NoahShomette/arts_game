use std::sync::{
    mpsc::{self, Receiver, Sender},
    Arc, Mutex,
};

use bevy::prelude::Resource;
use ehttp::Response;

#[derive(Resource, Clone)]
pub struct ReqwestClient {
    pub sender_channel: Sender<Result<Response, String>>,
    pub reciever_channel: Arc<Mutex<Receiver<Result<Response, String>>>>,
}

impl ReqwestClient {
    pub fn new() -> Self {
        let (sender, reciever) = mpsc::channel::<Result<Response, String>>();

        ReqwestClient {
            sender_channel: sender,
            reciever_channel: Arc::new(Mutex::new(reciever)),
        }
    }
}
