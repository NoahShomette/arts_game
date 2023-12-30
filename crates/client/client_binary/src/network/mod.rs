use bevy::app::Plugin;

use self::game_server_connection::GameServerPlugin;

mod game_server_connection;

pub struct NetworkPlugin;

impl Plugin for NetworkPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugins(GameServerPlugin);
    }
}
