//! The games menu does three things
//! - Show a player all games they are in
//! - Show a player all public open games they can join
//! - Create a new game
//!
//! The first two options should basically be tabs

use bevy::{
    app::Plugin,
    ecs::{
        component::Component,
        system::{Commands, Res},
    },
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

use crate::ui::{
    colors::CurrentColors,
    marker_component,
    scenes::ScenesAppExtension,
    widgets::tabbed_content::{tabbed_content, TabbedContentSettings},
};

use self::{
    game_entry_button::{game_entry_button, GameEntryButtonPlugin, GameEntryButtonStyle},
    games_data::GamesDataPlugin,
};

use super::MainMenuScreenState;

mod game_entry_button;
mod games_data;

pub struct GamesMenuPlugin;

impl Plugin for GamesMenuPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugins((GameEntryButtonPlugin, GamesDataPlugin));
        app.add_scene(
            marker_component::<GamesScreenRootMarker>(),
            setup_games,
            MainMenuScreenState::GamesPage,
        );
    }
}

#[derive(Component)]
pub struct GamesTabs;

#[derive(Component, TypePath)]
struct GamesScreenRootMarker;

#[derive(Component, TypePath)]
struct GameEntryMarker;

fn setup_games(mut commands: Commands, colors: Res<CurrentColors>) {
    // root ui for entire screen
    let screen_container = commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::SpaceBetween,
                align_items: AlignItems::Center,
                position_type: PositionType::Absolute,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            ..default()
        })
        .insert(GamesScreenRootMarker)
        .id();

    // top area ui
    let top_ui = commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(15.0),
                justify_content: JustifyContent::SpaceEvenly,
                align_items: AlignItems::Center,
                position_type: PositionType::Relative,
                flex_direction: FlexDirection::Row,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent.spawn(
                TextBundle::from_section(
                    "Games",
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

    let (tab_root, tabs) = tabbed_content(
        GamesTabs,
        TabbedContentSettings {
            tabs: vec!["Games".to_string(), "Open Games".to_string()],
            open_tab: 0,
        },
        &colors,
        &mut commands,
    );

    for (entity, _) in tabs.iter() {
        let tab_content = game_entry_button(
            GameEntryMarker,
            GameEntryButtonStyle::default(),
            &mut commands,
            &colors,
        );

        commands.entity(*entity).push_children(&[tab_content]);
    }

    commands
        .entity(screen_container)
        .push_children(&[top_ui, tab_root]);
}
