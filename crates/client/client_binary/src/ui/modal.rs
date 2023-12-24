use bevy::{
    app::{Plugin, Update},
    ecs::{
        bundle::Bundle,
        component::Component,
        entity::Entity,
        query::{Changed, With},
        system::{Commands, Query},
    },
    hierarchy::{BuildChildren, DespawnRecursiveExt},
    prelude::default,
    render::color::Color,
    text::TextStyle,
    ui::{
        node_bundles::{ButtonBundle, NodeBundle, TextBundle},
        widget::Button,
        AlignItems, BackgroundColor, FlexDirection, FocusPolicy, Interaction, JustifyContent,
        PositionType, Style, UiRect, Val,
    },
};

use super::BasicButton;

pub struct ModalPlugin;

impl Plugin for ModalPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Update, modal_button_interaction);
    }
}

fn modal_button_interaction(
    mut interaction_query: Query<
        (&Interaction, &ModalCloseButtonMarker),
        (Changed<Interaction>, With<Button>),
    >,
    mut commands: Commands,
) {
    for (interaction, modal_close_button) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                commands.entity(modal_close_button.0).despawn_recursive();
            }
            Interaction::Hovered => {}
            Interaction::None => {}
        }
    }
}

#[allow(dead_code)]
pub struct ModalStyle<B: Bundle> {
    with_close_button: bool,
    close_button_bundle: Option<B>,
    modal_size: Option<(Val, Val)>,
}

/// A marker marking a modal close button. Contains a reference to the root modal entity
#[derive(Component)]
struct ModalCloseButtonMarker(Entity);

/// Construct and spawn a new modal
#[allow(dead_code)]
fn modal_panel<T>(
    menu_type: T,
    modal_style: ModalStyle<impl Bundle>,
    commands: &mut Commands,
) -> Entity
where
    T: Component,
{
    //we assign it to a basic entity and then reassign it later
    let mut inside_entity = Entity::from_raw(0);
    //root node for the entire modal

    let modal_size = match modal_style.modal_size {
        None => (Val::Auto, Val::Auto),
        Some(size) => size,
    };

    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                position_type: PositionType::Absolute,
                ..default()
            },
            background_color: Color::BLACK.with_a(0.5).into(),
            focus_policy: FocusPolicy::Block,
            ..default()
        })
        .insert(menu_type)
        .with_children(|master_parent| {
            let parent_entity = master_parent.parent_entity();

            master_parent
                .spawn(NodeBundle {
                    style: Style {
                        width: modal_size.0,
                        height: modal_size.1,
                        padding: UiRect::all(Val::Px(5.0)),
                        justify_content: JustifyContent::Start,
                        align_items: AlignItems::Center,
                        position_type: PositionType::Relative,
                        flex_direction: FlexDirection::Column,
                        ..default()
                    },
                    background_color: Color::rgba(1.0, 1.0, 1.0, 1.0).into(),
                    ..default()
                })
                .with_children(|parent| {
                    //root node for the inside panel
                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                width: Val::Percent(100.0),
                                height: Val::Percent(100.0),
                                padding: UiRect::all(Val::Px(10.0)),
                                justify_content: JustifyContent::Start,
                                align_items: AlignItems::Center,
                                position_type: PositionType::Relative,
                                flex_direction: FlexDirection::Column,
                                ..default()
                            },
                            background_color: Color::rgba(0.0, 0.0, 0.0, 1.0).into(),
                            ..default()
                        })
                        .with_children(|parent| {
                            if modal_style.with_close_button {
                                // Top option close button
                                parent
                                    .spawn(NodeBundle {
                                        style: Style {
                                            width: Val::Percent(100.0),
                                            height: Val::Percent(10.0),
                                            justify_content: JustifyContent::End,
                                            align_items: AlignItems::Start,
                                            position_type: PositionType::Relative,
                                            flex_direction: FlexDirection::Row,
                                            ..default()
                                        },
                                        background_color: Color::rgba(0.0, 0.0, 0.0, 0.0).into(),
                                        ..default()
                                    })
                                    .with_children(|parent| {
                                        let mut button_entity = parent.spawn_empty();
                                        button_entity
                                            .insert(ButtonBundle {
                                                style: Style {
                                                    width: Val::Auto,
                                                    height: Val::Px(50.0),
                                                    margin: UiRect::new(
                                                        Val::Px(20.0),
                                                        Val::Px(20.0),
                                                        Val::Px(20.0),
                                                        Val::Px(20.0),
                                                    ),
                                                    padding: UiRect::all(Val::Px(10.0)),
                                                    justify_content: JustifyContent::Center,
                                                    align_items: AlignItems::Center,
                                                    ..Default::default()
                                                },
                                                background_color: BackgroundColor::from(
                                                    Color::GRAY,
                                                ),
                                                ..Default::default()
                                            })
                                            .insert(ModalCloseButtonMarker(parent_entity))
                                            .insert(BasicButton)
                                            .with_children(|parent| {
                                                parent.spawn(TextBundle::from_section(
                                                    "CLOSE",
                                                    TextStyle {
                                                        font_size: 40.0,
                                                        color: Color::BLACK,
                                                        ..default()
                                                    },
                                                ));
                                            });
                                        if let Some(bundle) = modal_style.close_button_bundle {
                                            button_entity.insert(bundle);
                                        }
                                    });
                            }

                            inside_entity = parent
                                .spawn(NodeBundle {
                                    style: Style {
                                        width: Val::Percent(100.0),
                                        height: Val::Percent(100.0),
                                        margin: UiRect::all(Val::Px(10.0)),
                                        justify_content: JustifyContent::Center,
                                        align_items: AlignItems::Center,
                                        position_type: PositionType::Relative,
                                        flex_direction: FlexDirection::Column,
                                        ..default()
                                    },
                                    background_color: Color::rgba(0.0, 0.0, 0.0, 0.0).into(),
                                    ..default()
                                })
                                .id();
                        });
                });
        });

    inside_entity
}
