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

use crate::ui::{colors::CurrentColors, UI_MODAL_LAYER};

use super::basic_button::BasicButton;

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
    pub with_close_button: bool,
    pub close_button_bundle: Option<B>,
    pub modal_size: Option<(Val, Val)>,
    pub outline: bool,
}

/// A marker marking a modal close button. Contains a reference to the root modal entity
#[derive(Component)]
struct ModalCloseButtonMarker(Entity);

/// Construct and spawn a new modal
#[allow(dead_code)]
pub fn modal_panel<T>(
    menu_type: T,
    modal_style: ModalStyle<impl Bundle>,
    colors: &CurrentColors,
    commands: &mut Commands,
) -> Entity
where
    T: Component,
{
    let modal_size = match modal_style.modal_size {
        None => (Val::Auto, Val::Auto),
        Some(size) => size,
    };

    // Root level node, spanning the whole screen and applying a 50% opacity
    let root = commands
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
            z_index: bevy::ui::ZIndex::Global(UI_MODAL_LAYER),
            ..default()
        })
        .insert(menu_type)
        .id();

    let mut border_or_root_entity = root;

    // Node behaving as a border for the actual modal
    if modal_style.outline {
        let border_entity = commands
            .spawn(NodeBundle {
                style: Style {
                    width: modal_size.0,
                    height: modal_size.1,
                    padding: UiRect::all(Val::Px(25.0)),
                    justify_content: JustifyContent::Start,
                    align_items: AlignItems::Center,
                    position_type: PositionType::Relative,
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
                background_color: colors.accent().into(),
                ..default()
            })
            .id();

        commands.entity(root).push_children(&[border_entity]);

        border_or_root_entity = border_entity;
    }

    //root node for the inside panel, padding to give it a border
    let inside_background_panel = commands
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
            background_color: colors.background().into(),
            ..default()
        })
        .id();
    if modal_style.with_close_button {
        // Top option close button
        let close_button = commands
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
                        background_color: BackgroundColor::from(colors.accent()),
                        ..Default::default()
                    })
                    .insert(ModalCloseButtonMarker(root))
                    .insert(BasicButton)
                    .with_children(|parent| {
                        parent.spawn(TextBundle::from_section(
                            "CLOSE",
                            TextStyle {
                                font_size: 40.0,
                                color: colors.dark_text(),
                                ..default()
                            },
                        ));
                    });
                if let Some(bundle) = modal_style.close_button_bundle {
                    button_entity.insert(bundle);
                }
            })
            .id();

        commands
            .entity(inside_background_panel)
            .push_children(&[close_button]);
    }

    // content entity is the entity to which all content should be children of. We assign it to a basic entity and then reassign it later
    let content_entity = commands
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

    commands
        .entity(border_or_root_entity)
        .push_children(&[inside_background_panel]);
    commands
        .entity(inside_background_panel)
        .push_children(&[content_entity]);
    content_entity
}
