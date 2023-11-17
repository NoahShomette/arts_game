use std::net::SocketAddr;

use bevy::prelude::Resource;
use tide::{
    http::headers::HeaderValue,
    security::{CorsMiddleware, Origin},
    Server,
};

pub mod authentication;

/// A resource to hold the Tide Server during plugin construction. Is started at the end of the app plugin cycle
#[derive(Resource)]
pub struct TideServerResource(pub Server<()>, pub SocketAddr);

impl TideServerResource {
    pub fn new(addr: SocketAddr) -> Self {
        let mut tide = tide::new();
        tide.with(
            CorsMiddleware::new()
                .allow_methods("GET, POST, OPTIONS".parse::<HeaderValue>().unwrap())
                .allow_origin(Origin::from("*")),
        );
        TideServerResource(tide, addr)
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
