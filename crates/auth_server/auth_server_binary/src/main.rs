use arts_authentication::ServerLibraryPlugin;
use bevy::{prelude::App, MinimalPlugins};
use clap::Parser;
use core_library::http_server::TideServerResource;
use tide::http::url;

#[allow(clippy::unwrap_used)]
#[derive(Parser)]
struct ServerArgs {
    #[arg(short, long)]
    address: String,
    #[arg(long)]
    http_port: u16,
}

fn main() {
    let cli = ServerArgs::parse();
    let http_server_addr = url::Url::parse(&format!(
        "http://{}:{}",
        cli.address.clone(),
        &cli.http_port.to_string()
    ))
    .expect("Invalid address given");
    println!("{}", http_server_addr);
    let mut app = App::new();

    app.insert_resource(TideServerResource::new(http_server_addr));
    app.add_plugins(MinimalPlugins);
    // --- All custom plugins should go here
    app.add_plugins(ServerLibraryPlugin);
    // ---
    // Must be the last items called starting the server
    let tide = app
        .world
        .remove_resource::<TideServerResource>()
        .expect("TideServerResource expected to start server");
    tide.start_server();
    app.run();
}
