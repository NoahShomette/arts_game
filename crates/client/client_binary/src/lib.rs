mod network;
pub mod ui;

use core_library::authentication::client_authentication::{PasswordLoginInfo, SignInEvent};
use core_library::TaskPoolRes;
use bevy::app::Update;
use bevy::ecs::event::EventWriter;
use bevy::{app::Plugin, tasks::TaskPoolBuilder};
use network::NetworkPlugin;

pub struct ClientPlugin;

impl Plugin for ClientPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugins(NetworkPlugin);

        app.insert_resource(TaskPoolRes(TaskPoolBuilder::new().num_threads(2).build()));
        app.add_systems(Update, sign_in);
    }
}

fn sign_in(mut su: EventWriter<SignInEvent>) {
    su.send(SignInEvent {
        login_info: PasswordLoginInfo::new("noahshomette@gmail.com", "123456", true),
    });
}

fn request_games(mut su: EventWriter<SignInEvent>) {
    su.send(SignInEvent {
        login_info: PasswordLoginInfo::new("noahshomette@gmail.com", "123456", true),
    });
}
