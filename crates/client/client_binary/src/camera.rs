use bevy::{
    app::{Plugin, Startup},
    core_pipeline::core_2d::Camera2dBundle,
    ecs::system::Commands,
};

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Startup, spawn_camera);
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
