use bevy::{app::Plugin, tasks::TaskPoolBuilder};
use bevy_eventwork::EventworkRuntime;
use bevy_eventwork_mod_websockets::{NetworkSettings, WebSocketProvider};

pub struct GameServerPlugin;

impl Plugin for GameServerPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugins(bevy_eventwork::EventworkPlugin::<
            WebSocketProvider,
            bevy::tasks::TaskPool,
        >::default());
        app.insert_resource(NetworkSettings::default());
        app.insert_resource(EventworkRuntime(
            TaskPoolBuilder::new().num_threads(2).build(),
        ));
    }
}
