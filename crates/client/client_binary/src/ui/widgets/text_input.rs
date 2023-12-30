use bevy::{
    app::{Plugin, Update},
    ecs::{
        entity::Entity,
        query::Changed,
        system::{Query, Res},
    },
    render::color::Color,
    ui::{BorderColor, Interaction},
};
use bevy_simple_text_input::TextInput;

use crate::ui::colors::CurrentColors;

pub struct CustomTextInputPlugin;

impl Plugin for CustomTextInputPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Update, focus);
    }
}

fn focus(
    query: Query<(Entity, &Interaction), Changed<Interaction>>,
    mut text_input_query: Query<(Entity, &mut TextInput, &mut BorderColor)>,
    colors: Res<CurrentColors>,
) {
    for (interaction_entity, interaction) in &query {
        if *interaction == Interaction::Pressed {
            for (entity, mut text_input, mut border_color) in &mut text_input_query {
                if entity == interaction_entity {
                    text_input.inactive = false;
                    *border_color = Color::BLACK.into();
                } else {
                    text_input.inactive = true;
                    *border_color = colors.accent().into();
                }
            }
        }
    }
}
