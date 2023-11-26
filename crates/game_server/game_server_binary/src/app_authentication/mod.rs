//! Is responsible for authenticating the server. Eventually the server will require an account that has the Server role. This
//! will allow the server to also make http requests to change data for the data it owns. Eg delete a game, change a game ip, etc.

use bevy::app::Plugin;

pub struct AuthenticationPlugin;

impl Plugin for AuthenticationPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugins(arts_core::authentication::CoreAuthenticationPlugin);
    }
}
