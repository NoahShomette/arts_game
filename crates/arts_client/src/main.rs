use arts_client::ClientPlugin;
use bevy::app::App;

use bevy::DefaultPlugins;

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins);
    app.add_plugins(ClientPlugin);
    app.run();
}
