use bevy::{
    app::Plugin,
    ecs::{component::Component, system::Commands},
    hierarchy::BuildChildren,
    prelude::default,
    reflect::TypePath,
    render::color::Color,
    text::{TextAlignment, TextStyle},
    ui::{
        node_bundles::{NodeBundle, TextBundle},
        AlignItems, FlexDirection, JustifyContent, PositionType, Style, UiRect, Val,
    },
};

use crate::{app::AppState, ui::marker_component};

use self::{authenticate::AuthenticatePlugin, username::UsernameUiPlugin};

use super::ScenesAppExtension;

mod authenticate;
mod games;
mod home;
mod username;

pub struct UiMainMenuPlugin;

impl Plugin for UiMainMenuPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugins(UsernameUiPlugin);
        app.add_scene(
            marker_component::<MainMenuRootMarker>(),
            setup_main_menu_ui,
            AppState::MainMenu,
        );
        app.add_plugins(AuthenticatePlugin);
    }
}

/// The core different main screens that the main menu can have
pub enum MainMenuScreenState {
    None,
    AuthenticatePage,
    HomePage,
    GamesPage,
    SettingsPage,
}

#[derive(Component, TypePath)]
struct MainMenuRootMarker;

fn setup_main_menu_ui(mut commands: Commands) {
    // root ui for entire screen
    let screen_container = commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::SpaceBetween,
                align_items: AlignItems::Center,
                position_type: PositionType::Absolute,
                ..default()
            },
            ..default()
        })
        .insert(MainMenuRootMarker)
        .id();

    // Left area ui
    let left_ui = commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(50.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::SpaceEvenly,
                align_items: AlignItems::Center,
                position_type: PositionType::Relative,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent.spawn(
                TextBundle::from_section(
                    "Arts Game",
                    TextStyle {
                        font_size: 100.0,
                        color: Color::WHITE,
                        ..default()
                    },
                )
                .with_text_alignment(TextAlignment::Center)
                .with_style(Style {
                    width: Val::Percent(50.0),
                    height: Val::Percent(100.0),
                    position_type: PositionType::Relative,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    margin: UiRect::top(Val::Px(75.0)),
                    ..default()
                }),
            );
        })
        .id();

    commands.entity(screen_container).push_children(&[left_ui]);
}
