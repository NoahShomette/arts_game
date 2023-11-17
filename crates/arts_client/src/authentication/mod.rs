use bevy::app::Plugin;

pub struct AuthenticationPlugin;

impl Plugin for AuthenticationPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugins(arts_core::authentication::client_authentication::CoreAuthenticationPlugin);
    }
}
