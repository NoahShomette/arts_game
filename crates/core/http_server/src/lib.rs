use bevy::prelude::Resource;
use tide::{
    http::{headers::HeaderValue, Url},
    security::{CorsMiddleware, Origin},
    Error, Request, Server,
};

/// A resource to hold the Tide Server during plugin construction. Is started at the end of the app plugin cycle
#[derive(Resource)]
pub struct TideServerResource(pub Server<()>, pub Url);

impl TideServerResource {
    pub fn new(addr: Url) -> Self {
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

async fn start_server(tide: Server<()>, address: Url) -> tide::Result<()> {
    Ok(tide.listen(address).await?)
}

pub fn request_access_token(req: &Request<()>) -> Result<String, Error> {
    let Some(access_token) = req.header("authorization") else {
        return Err(Error::from_str(400, "No Authorization Bearer found"));
    };
    let string_at = access_token.to_string();
    Ok(string_at.split_whitespace().collect::<Vec<&str>>()[1].to_string())
}
