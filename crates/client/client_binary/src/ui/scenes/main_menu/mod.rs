use bevy::{
    app::Plugin,
    ecs::{
        schedule::{NextState, OnEnter, OnExit, States},
        system::ResMut,
    },
};

use crate::app::AppState;

use self::{
    authenticate::AuthenticatePlugin, games_screen::GamesMenuPlugin, home::MainMenuHomePlugin,
    username::UsernameUiPlugin,
};

mod authenticate;
mod games_screen;
mod home;
mod username;

pub struct UiMainMenuPlugin;

impl Plugin for UiMainMenuPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_state::<MainMenuScreenState>();
        app.add_plugins(UsernameUiPlugin);

        app.add_plugins((AuthenticatePlugin, MainMenuHomePlugin, GamesMenuPlugin));

        app.add_systems(
            OnEnter(AppState::MainMenu),
            |mut next_state: ResMut<NextState<MainMenuScreenState>>| {
                next_state.set(MainMenuScreenState::HomePage)
            },
        );

        app.add_systems(
            OnExit(AppState::MainMenu),
            |mut next_state: ResMut<NextState<MainMenuScreenState>>| {
                next_state.set(MainMenuScreenState::None)
            },
        );
    }
}

/// The core different main screens that the main menu can have
#[derive(States, Default, Debug, PartialEq, Eq, Hash, Clone)]
pub enum MainMenuScreenState {
    #[default]
    None,
    HomePage,
    GamesPage,
    SettingsPage,
}
