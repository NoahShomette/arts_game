use bevy::app::Plugin;

pub struct AuthenticationPlugin;

impl Plugin for AuthenticationPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugins(core_library::authentication::CoreAuthenticationPlugin);
    }
}
