mod network;
pub mod ui;

use arts_core::authentication::client_authentication::{PasswordLoginInfo, SignInEvent};
use arts_core::{authentication::client_authentication::AuthClient, TaskPoolRes};
use bevy::app::Update;
use bevy::ecs::event::EventWriter;
use bevy::{app::Plugin, tasks::TaskPoolBuilder};
use network::NetworkPlugin;

pub struct ClientPlugin;

impl Plugin for ClientPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_resource(AuthClient::new());
        app.add_plugins(NetworkPlugin);

        app.insert_resource(TaskPoolRes(TaskPoolBuilder::new().num_threads(2).build()));
        app.add_systems(Update, sign_up);
    }
}

fn sign_up(mut su: EventWriter<SignInEvent>) {
    su.send(SignInEvent {
        login_info: PasswordLoginInfo::new("noahshomette@gmail.com", "123456"),
    });
}
