use arts_client::ClientPlugin;
use bevy::app::App;

use bevy::DefaultPlugins;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_tweening::TweeningPlugin;

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins);
    app.add_plugins(TweeningPlugin);
    app.add_plugins(ClientPlugin);
    app.add_plugins(WorldInspectorPlugin::new());

    app.run();
}
