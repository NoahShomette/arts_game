use std::{
    net::{Ipv4Addr, SocketAddr, SocketAddrV4},
    str::FromStr,
};

use arts_core::http_server::TideServerResource;
use arts_server::ServerPlugin;
use bevy::{
    app::App,
    time::{Fixed, Time},
    MinimalPlugins,
};
use clap::Parser;

#[derive(Parser)]
struct ServerArgs {
    #[arg(short, long)]
    address: String,
    #[arg(short, long)]
    port: u16,
}

fn main() {
    let cli = ServerArgs::parse();
    let server_addr = SocketAddr::V4(SocketAddrV4::new(
        Ipv4Addr::from_str(&cli.address).expect("Invalid IP Address"),
        cli.port,
    ));
    println!("{}", server_addr);
    let mut app = App::new();
    app.insert_resource(TideServerResource::new(server_addr));
    app.insert_resource(Time::<Fixed>::from_seconds(1.0));
    app.add_plugins((MinimalPlugins, bevy::log::LogPlugin::default()));

    app.add_plugins(ServerPlugin);

    // Must be the last items called starting the server
    let tide = app
        .world
        .remove_resource::<TideServerResource>()
        .expect("TideServerResource expected to start server");
    tide.start_server();

    app.run();
}
