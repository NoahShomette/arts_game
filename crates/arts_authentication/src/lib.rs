use std::net::SocketAddr;

use bevy::prelude::Resource;
use tide::Server;

pub mod authentication;
pub mod http_relay_server;

/// A resource to hold the Tide Server during plugin construction. Is started at the end of the app plugin cycle
#[derive(Resource)]
pub struct TideServerResource(pub Server<()>, pub SocketAddr);

impl TideServerResource {
    pub fn new(addr: SocketAddr) -> Self {
        TideServerResource(tide::new(), addr)
    }
    pub fn start_server(self) {
        bevy::tasks::IoTaskPool::get()
            .spawn(start_server(self.0, self.1.clone()))
            .detach();
    }
}

async fn start_server(tide: Server<()>, address: SocketAddr) -> tide::Result<()> {
    Ok(tide.listen(address).await?)
}
