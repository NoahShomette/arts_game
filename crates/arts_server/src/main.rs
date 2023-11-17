use arts_server::ServerPlugin;
use bevy::{
    app::App,
    tasks::TaskPoolBuilder,
    time::{Fixed, Time},
    MinimalPlugins,
};
use bevy_eventwork::EventworkRuntime;

fn main() {
    let mut app = App::new();
    app.insert_resource(Time::<Fixed>::from_seconds(1.0));
    app.add_plugins((MinimalPlugins, bevy::log::LogPlugin::default()));

    app.add_plugins(ServerPlugin);

    app.insert_resource(EventworkRuntime(
        TaskPoolBuilder::new().num_threads(2).build(),
    ));

    app.run();
}
