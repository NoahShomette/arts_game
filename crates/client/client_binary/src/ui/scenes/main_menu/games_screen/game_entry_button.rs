use std::{ops::Sub, time::Duration};

use bevy::{
    app::{Plugin, Update},
    ecs::{
        component::Component,
        entity::Entity,
        query::{Changed, With, Without},
        system::{Commands, Query, Res},
    },
    hierarchy::BuildChildren,
    math::Vec3,
    prelude::default,
    text::TextStyle,
    transform::components::Transform,
    ui::{
        node_bundles::{ButtonBundle, NodeBundle, TextBundle},
        widget::Button,
        AlignItems, BackgroundColor, BorderColor, Interaction, JustifyContent, PositionType, Style,
        UiRect, Val,
    },
};
use bevy_tweening::{lens::TransformScaleLens, Animator, EaseFunction, RepeatCount, Tween};
use chrono::{DateTime, Utc};
use core_library::network::ServerType;

use crate::ui::{colors::CurrentColors, widgets::basic_button::DisabledButton};

pub struct GameEntryButtonPlugin;

impl Plugin for GameEntryButtonPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Update, handle_button_visuals);
    }
}

/* Dont currently need this but I do assume it will be needed eventually
#[derive(Component)]
pub struct SelectedButton;
*/

#[derive(Component)]
pub struct GameEntryButton;

fn handle_button_visuals(
    mut interaction_query: Query<
        (
            Entity,
            &Transform,
            &Interaction,
            &mut BorderColor,
            &mut BackgroundColor,
        ),
        (
            Changed<Interaction>,
            With<Button>,
            Without<DisabledButton>,
            With<GameEntryButton>,
        ),
    >,
    mut commands: Commands,
    colors: Res<CurrentColors>,
) {
    for (entity, transform, interaction, mut border_color, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *color = BackgroundColor::from(colors.highlight());
            }
            Interaction::Hovered => {
                let transform_tween = Tween::new(
                    EaseFunction::QuadraticInOut,
                    Duration::from_millis(50),
                    TransformScaleLens {
                        start: transform.scale,

                        end: Vec3 {
                            x: 1.01,
                            y: 1.01,
                            z: 1.0,
                        },
                    },
                )
                .with_repeat_count(RepeatCount::Finite(1));

                commands
                    .entity(entity)
                    .try_insert(Animator::new(transform_tween));

                *border_color = colors.highlight().into();
            }
            Interaction::None => {
                let transform_tween = Tween::new(
                    EaseFunction::QuadraticInOut,
                    Duration::from_millis(50),
                    TransformScaleLens {
                        start: transform.scale,
                        end: Vec3 {
                            x: 1.0,
                            y: 1.0,
                            z: 1.0,
                        },
                    },
                )
                .with_repeat_count(RepeatCount::Finite(1));

                commands
                    .entity(entity)
                    .try_insert(Animator::new(transform_tween));

                *color = BackgroundColor::from(colors.background_dark());
                *border_color = colors.accent().into();
            }
        }
    }
}

pub struct GameEntryButtonStyle {
    pub game_name: String,
    pub player_count: u8,
    pub max_player_count: u8,
    pub server_type: ServerType,
    pub started_time: Option<DateTime<Utc>>,
}

impl Default for GameEntryButtonStyle {
    fn default() -> Self {
        Self {
            game_name: String::from("Arts Game"),
            player_count: 0,
            max_player_count: 10,
            server_type: ServerType::PlayerHosted,
            started_time: Some(Utc::now().sub(chrono::Duration::hours(47))),
        }
    }
}

pub fn game_entry_button<T>(
    button_marker: T,
    button_style: GameEntryButtonStyle,
    commands: &mut Commands,
    colors: &CurrentColors,
) -> Entity
where
    T: Component,
{
    let entity = commands
        .spawn((
            ButtonBundle {
                style: Style {
                    width: Val::Percent(75.0),
                    height: Val::Px(100.0),
                    margin: UiRect::new(Val::Px(20.0), Val::Px(20.0), Val::Px(20.0), Val::Px(20.0)),
                    padding: UiRect::all(Val::Px(10.0)),
                    justify_content: JustifyContent::SpaceBetween,
                    align_items: AlignItems::Center,
                    flex_direction: bevy::ui::FlexDirection::Row,
                    border: UiRect::all(Val::Px(5.0)),
                    ..Default::default()
                },
                border_color: colors.accent().into(),
                background_color: BackgroundColor::from(colors.background_dark()),
                ..Default::default()
            },
            button_marker,
            GameEntryButton,
        ))
        .id();

    let name_and_players = commands
        .spawn((NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                position_type: PositionType::Relative,
                flex_direction: bevy::ui::FlexDirection::Column,
                ..default()
            },
            ..default()
        },))
        .id();

    commands.entity(name_and_players).with_children(|parent| {
        parent.spawn(TextBundle::from_section(
            &button_style.game_name,
            TextStyle {
                font_size: 40.0,
                color: colors.light_text(),
                ..default()
            },
        ));

        parent.spawn(TextBundle::from_section(
            format!(
                "Players: {}/{}",
                button_style.player_count, button_style.max_player_count
            ),
            TextStyle {
                font_size: 20.0,
                color: colors.light_text(),
                ..default()
            },
        ));
    });

    let game_center = commands
        .spawn((NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                position_type: PositionType::Relative,
                flex_direction: bevy::ui::FlexDirection::Column,
                border: UiRect::horizontal(Val::Px(5.0)),
                ..default()
            },
            border_color: colors.accent().into(),
            ..default()
        },))
        .id();

    let game_info = commands
        .spawn((NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                position_type: PositionType::Relative,
                flex_direction: bevy::ui::FlexDirection::Column,
                ..default()
            },
            ..default()
        },))
        .id();

    match button_style.started_time {
        Some(start_time) => {
            let game_duration = Utc::now().sub(start_time);
            let days = game_duration.num_days();
            let remained_hours = game_duration.num_hours() - (days * 24);

            commands.entity(game_info).with_children(|parent| {
                parent.spawn(TextBundle::from_section(
                    format!("Game Length: {} days {} hours", days, remained_hours),
                    TextStyle {
                        font_size: 20.0,
                        color: colors.light_text(),
                        ..default()
                    },
                ));
            });
        }
        None => {
            commands.entity(game_info).with_children(|parent| {
                parent.spawn(TextBundle::from_section(
                    "Game Not Started",
                    TextStyle {
                        font_size: 20.0,
                        color: colors.light_text(),
                        ..default()
                    },
                ));
            });
        }
    };

    commands.entity(entity).push_children(&[name_and_players]);
    commands.entity(entity).push_children(&[game_center]);
    commands.entity(entity).push_children(&[game_info]);

    entity
}
