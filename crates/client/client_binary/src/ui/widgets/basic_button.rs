use std::time::Duration;

use bevy::{
    app::{App, Plugin, Update},
    ecs::{
        bundle::Bundle,
        component::Component,
        entity::Entity,
        query::{Changed, With, Without},
        system::{Commands, IntoSystem, Query, Res},
    },
    hierarchy::{BuildChildren, Children},
    math::Vec3,
    prelude::default,
    reflect::TypePath,
    text::{Text, TextStyle},
    ui::{
        node_bundles::{ButtonBundle, TextBundle},
        widget::Button,
        AlignItems, BackgroundColor, Interaction, JustifyContent, Style, UiRect, Val,
    },
};
use bevy_tweening::{
    lens::TransformScaleLens, Animator, EaseFunction, RepeatCount, RepeatStrategy, Tween,
};

use crate::ui::{colors::CurrentColors, MarkerComponent, UiSystemIdResource};

pub struct BasicButtonPlugin;

impl Plugin for BasicButtonPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Update, handle_button_visuals);
    }
}

pub trait BasicButtonAppExtension {
    /// Adds a system that will one shot run the system with the given [`bevy::ecs::system::SystemId`] whenever a [`BasicButton`] that has this specific marker component is pressed.
    ///
    /// See [`basic_button_interaction`] for the actual system that runs.
    fn add_button_handler<Marker: Component + TypePath, M>(
        &mut self,
        _: MarkerComponent<Marker>,

        system: impl IntoSystem<(), (), M> + 'static,
    );
}

impl BasicButtonAppExtension for App {
    fn add_button_handler<Marker: Component + TypePath, M>(
        &mut self,
        _: MarkerComponent<Marker>,

        system: impl IntoSystem<(), (), M> + 'static,
    ) {
        let system_id = self.world.register_system(system);
        let mut resource = self.world.resource_mut::<UiSystemIdResource>();
        resource.map.insert(Marker::type_path(), system_id);
        self.add_systems(Update, basic_button_interaction::<Marker>);
    }
}

#[derive(Component)]
pub struct DisabledButton;

/* Dont currently need this but I do assume it will be needed eventually
#[derive(Component)]
pub struct SelectedButton;
*/

#[derive(Component)]
pub struct BasicButton;

fn handle_button_visuals(
    mut interaction_query: Query<
        (Entity, &Interaction, &mut BackgroundColor, &Children),
        (
            Changed<Interaction>,
            With<Button>,
            Without<DisabledButton>,
            With<BasicButton>,
        ),
    >,
    mut children_text_color: Query<&mut Text>,
    mut commands: Commands,
    colors: Res<CurrentColors>,
) {
    for (entity, interaction, mut color, children) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                let transform_tween = Tween::new(
                    EaseFunction::QuadraticInOut,
                    Duration::from_millis(50),
                    TransformScaleLens {
                        start: Vec3 {
                            x: 1.0,
                            y: 1.0,
                            z: 1.0,
                        },

                        end: Vec3 {
                            x: 1.4,
                            y: 1.0,
                            z: 1.0,
                        },
                    },
                )
                .with_repeat_count(RepeatCount::Finite(2))
                .with_repeat_strategy(RepeatStrategy::MirroredRepeat);

                commands
                    .entity(entity)
                    .try_insert(Animator::new(transform_tween));
            }
            Interaction::Hovered => {
                let transform_tween = Tween::new(
                    EaseFunction::QuadraticInOut,
                    Duration::from_millis(100),
                    TransformScaleLens {
                        start: Vec3 {
                            x: 1.0,
                            y: 1.0,
                            z: 1.0,
                        },

                        end: Vec3 {
                            x: 1.1,
                            y: 1.1,
                            z: 1.0,
                        },
                    },
                )
                .with_repeat_count(RepeatCount::Finite(2))
                .with_repeat_strategy(RepeatStrategy::MirroredRepeat);

                commands
                    .entity(entity)
                    .try_insert(Animator::new(transform_tween));

                *color = BackgroundColor::from(colors.highlight());
                for &child in children.iter() {
                    if let Ok(mut text) = children_text_color.get_mut(child) {
                        text.sections[0].style.color = colors.light_text();
                    }
                }
            }
            Interaction::None => {
                *color = BackgroundColor::from(colors.background_dark());
                for &child in children.iter() {
                    if let Ok(mut text) = children_text_color.get_mut(child) {
                        text.sections[0].style.color = colors.dark_text();
                    }
                }
            }
        }
    }
}

pub fn basic_button_interaction<MarkerComponent: Component + TypePath>(
    mut interaction_query: Query<
        (&Interaction, &MarkerComponent),
        (
            Changed<Interaction>,
            (With<Button>, With<BasicButton>, Without<DisabledButton>),
        ),
    >,
    mut commands: Commands,
    resource: Res<UiSystemIdResource>,
) {
    for (interaction, _) in &mut interaction_query {
        let system_id = resource.map.get(MarkerComponent::type_path()).unwrap();
        if let Interaction::Pressed = *interaction {
            commands.run_system(*system_id);
        }
    }
}

pub struct ButtonStyle<B: Bundle> {
    pub bundle: Option<B>,
    pub text: String,
    pub font_size: f32,
}

impl<B> Default for ButtonStyle<B>
where
    B: Bundle,
{
    fn default() -> Self {
        Self {
            bundle: Default::default(),
            text: Default::default(),
            font_size: 40.0,
        }
    }
}

pub fn basic_button<T>(
    button_marker: T,
    button_style: ButtonStyle<impl Bundle>,
    commands: &mut Commands,
    colors: &CurrentColors,
) -> Entity
where
    T: Component,
{
    let mut entity = commands.spawn((
        ButtonBundle {
            style: Style {
                width: Val::Auto,
                height: Val::Px(button_style.font_size + 10.0),
                margin: UiRect::new(Val::Px(20.0), Val::Px(20.0), Val::Px(20.0), Val::Px(20.0)),
                padding: UiRect::all(Val::Px(10.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            background_color: BackgroundColor::from(colors.background_dark()),
            ..Default::default()
        },
        button_marker,
        BasicButton,
    ));

    entity.with_children(|parent| {
        parent.spawn(TextBundle::from_section(
            &button_style.text,
            TextStyle {
                font_size: button_style.font_size,
                color: colors.light_text(),
                ..default()
            },
        ));
    });

    if let Some(bundle) = button_style.bundle {
        entity.insert(bundle);
    };

    entity.id()
}
