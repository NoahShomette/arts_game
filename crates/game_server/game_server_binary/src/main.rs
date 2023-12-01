use arts_server::ServerPlugin;
use bevy::{
    app::App,
    time::{Fixed, Time},
    MinimalPlugins,
};
use clap::Parser;
use console_parser::ConsoleParserPlugin;
use core_library::{http_server::TideServerResource, network::GameAddrInfo};

#[derive(Parser)]
struct ServerArgs {
    #[arg(short, long)]
    address: String,
    #[arg(long)]
    http_port: u16,
    #[arg(long)]
    ws_port: u16,
}

fn main() {
    let cli = ServerArgs::parse();
    let server_connect_info = GameAddrInfo {
        server_addr: cli.address,
        http_port: cli.http_port,
        ws_port: cli.ws_port,
    };
    let http_server_addr = server_connect_info.http_url();
    println!("{}", http_server_addr);
    let mut app = App::new();
    app.insert_resource(server_connect_info);
    app.insert_resource(TideServerResource::new(http_server_addr));
    app.insert_resource(Time::<Fixed>::from_seconds(1.0));
    app.add_plugins((MinimalPlugins, bevy::log::LogPlugin::default()));
    app.add_plugins(ServerPlugin);
    app.add_plugins(ConsoleParserPlugin);

    // Must be the last items called starting the server
    let tide = app
        .world
        .remove_resource::<TideServerResource>()
        .expect("TideServerResource expected to start server");
    tide.start_server();

    app.run();
}
