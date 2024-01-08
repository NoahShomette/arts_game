use bevy::{
    app::{Plugin, Update},
    ecs::{
        change_detection::DetectChanges,
        component::Component,
        entity::Entity,
        query::{With, Without},
        system::{Commands, Query},
        world::Ref,
    },
    hierarchy::{BuildChildren, Children, Parent},
    prelude::default,
    ui::{
        node_bundles::NodeBundle, widget::Button, AlignItems, Display, FlexDirection, FocusPolicy,
        Interaction, JustifyContent, PositionType, Style, UiRect, Val,
    },
};

use crate::ui::colors::CurrentColors;

use super::basic_button::{basic_button, BasicButton, ButtonStyle, DisabledButton};

pub struct TabbedContentPlugin;

impl Plugin for TabbedContentPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Update, tab_button_interaction);
    }
}

pub fn tab_button_interaction(
    tab_buttons: Query<
        (Ref<Interaction>, &TabContentEntity, &Parent),
        ((With<Button>, With<BasicButton>, Without<DisabledButton>),),
    >,
    children: Query<&Children>,
    mut tab_content: Query<(&mut Style, &TabContent), Without<TabContentEntity>>,
) {
    for (interaction, tab_content_entity, parent) in tab_buttons.iter() {
        if !interaction.is_changed() {
            continue;
        }
        if let Interaction::Pressed = *interaction {
            if let Ok(parent_children) = children.get(parent.get()) {
                for child in parent_children.iter() {
                    let Ok((_, other_tab_content_entity, _)) = tab_buttons.get(*child) else {
                        continue;
                    };

                    if let Ok((mut style, _)) = tab_content.get_mut(other_tab_content_entity.0) {
                        style.display = Display::None;
                    }
                }
            }

            if let Ok((mut style, _)) = tab_content.get_mut(tab_content_entity.0) {
                style.display = Display::DEFAULT;
            }
        }
    }
}

#[derive(Component)]
pub struct TabContentEntity(Entity);

#[derive(Component)]
pub struct TabContent;

#[derive(Component)]
pub struct TabContentButton;

/// Settings used to construct a tabbed view
pub struct TabbedContentSettings {
    /// Each tab needs a unique identifier
    pub tabs: Vec<String>,
    pub open_tab: usize,
}

/// Construct and spawn a new tabbed content container. Returns the entity for the entire tabbed content and the entity for each tab.
pub fn tabbed_content<T>(
    menu_type: T,
    tab_settings: TabbedContentSettings,
    colors: &CurrentColors,
    commands: &mut Commands,
) -> (Entity, Vec<(Entity, String)>)
where
    T: Component,
{
    let mut tab_entities = Vec::with_capacity(tab_settings.tabs.len());

    let root = commands
        .spawn((NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                position_type: PositionType::Relative,
                flex_direction: FlexDirection::Column,
                border: UiRect::all(Val::Px(10.0)),
                ..default()
            },
            background_color: colors.background().into(),
            border_color: colors.accent().into(),
            focus_policy: FocusPolicy::Block,
            ..default()
        },))
        .insert(menu_type)
        .id();

    let tab_buttons = commands
        .spawn((NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(10.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                position_type: PositionType::Relative,
                flex_direction: FlexDirection::Row,
                ..default()
            },
            ..default()
        },))
        .id();

    let tab_contents = commands
        .spawn((NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                position_type: PositionType::Relative,
                border: UiRect::top(Val::Px(5.0)),
                ..default()
            },
            border_color: colors.accent().into(),
            ..default()
        },))
        .id();

    for (i, tab) in tab_settings.tabs.iter().enumerate() {
        let display = if i == tab_settings.open_tab {
            Display::DEFAULT
        } else {
            Display::None
        };
        let content = commands
            .spawn((
                NodeBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        height: Val::Percent(90.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        position_type: PositionType::Relative,
                        display,
                        ..default()
                    },
                    ..default()
                },
                TabContent,
            ))
            .id();

        tab_entities.push((content, tab.clone()));

        let button_style = ButtonStyle {
            bundle: Some(TabContentEntity(content)),
            text: tab.clone(),
            font_size: 40.0,
        };
        let button = basic_button(TabContentButton, button_style, commands, colors);

        commands.entity(tab_buttons).push_children(&[button]);
        commands.entity(tab_contents).push_children(&[content]);
    }

    commands
        .entity(root)
        .push_children(&[tab_buttons, tab_contents]);

    (root, tab_entities)
}
