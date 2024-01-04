use bevy::app::Plugin;

use self::{
    basic_button::BasicButtonPlugin, modal::ModalPlugin, tabbed_content::TabbedContentPlugin,
    text_input::CustomTextInputPlugin,
};

pub mod basic_button;
pub mod modal;
pub mod tabbed_content;
pub mod text_input;

pub struct WidgetsPlugin;

impl Plugin for WidgetsPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugins((
            ModalPlugin,
            BasicButtonPlugin,
            CustomTextInputPlugin,
            TabbedContentPlugin,
        ));
    }
}