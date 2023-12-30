use bevy::{
    app::Plugin,
    ecs::schedule::{States, SystemSet},
};

pub struct AppPlugin;

impl Plugin for AppPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_state::<AppState>();
    }
}

#[derive(States, Default, Clone, Eq, PartialEq, Debug, Hash)]
pub enum AppState {
    #[default]
    LoadingApp,
    MainMenu,
    LoadingGame,
    InGame,
}

/// Sets that are used for app level systems
#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
pub enum AppSets {}

/// sets that are used for in game level systems
#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
pub enum GameSets {}
