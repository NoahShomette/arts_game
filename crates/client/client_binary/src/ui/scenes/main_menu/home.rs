use bevy::{
    app::Plugin,
    ecs::{
        component::Component,
        schedule::NextState,
        system::{Commands, Res, ResMut},
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
    widgets::basic_button::{basic_button, BasicButtonAppExtension, ButtonStyle},
};

use super::MainMenuScreenState;

pub struct MainMenuHomePlugin;

impl Plugin for MainMenuHomePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_scene(
            marker_component::<MainMenuRootMarker>(),
            setup_main_menu_ui,
            MainMenuScreenState::HomePage,
        );

        app.add_button_handler(
            marker_component::<GamesButton>(),
            |mut next_state: ResMut<NextState<MainMenuScreenState>>| {
                next_state.set(MainMenuScreenState::GamesPage)
            },
        );
    }
}

#[derive(Component, TypePath)]
struct MainMenuRootMarker;

#[derive(Component, TypePath)]
struct GamesButton;

fn setup_main_menu_ui(mut commands: Commands, colors: Res<CurrentColors>) {
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

    let button_style = ButtonStyle {
        bundle: None::<()>,
        text: "Games".to_string(),
        font_size: 64.0,
    };
    let button = basic_button(GamesButton, button_style, &mut commands, &colors);
    commands.entity(left_ui).push_children(&[button]);

    commands.entity(screen_container).push_children(&[left_ui]);
}
