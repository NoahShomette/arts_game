use bevy::{app::Plugin, ecs::system::Resource, render::color::Color};

pub struct GameColorsPlugin;

impl Plugin for GameColorsPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.init_resource::<CurrentColors>();
    }
}

#[derive(Resource, Default)]
pub struct CurrentColors(GameColorPalette);

impl CurrentColors {
    pub fn dark_text(&self) -> Color {
        self.0.dark_text
    }
    pub fn light_text(&self) -> Color {
        self.0.light_text
    }
    pub fn background(&self) -> Color {
        self.0.background
    }
    pub fn background_dark(&self) -> Color {
        self.0.background_dark
    }
    pub fn accent(&self) -> Color {
        self.0.accent
    }
    pub fn highlight(&self) -> Color {
        self.0.highlight
    }
}

pub const MODAL_TEXT: Color = Color::rgb(0.12, 0.15, 0.10);
pub const MODAL_BACKGROUND: Color = Color::rgb(0.79, 0.72, 0.65);
pub const MODAL_ALTERNATE: Color = Color::rgb(0.69, 0.62, 0.55);

pub struct GameColorPalette {
    dark_text: Color,
    light_text: Color,
    background: Color,
    background_dark: Color,
    accent: Color,
    highlight: Color,
}

impl Default for GameColorPalette {
    fn default() -> Self {
        GameColorPalette::dark()
    }
}

impl GameColorPalette {
    pub fn dark() -> GameColorPalette {
        Self {
            dark_text: Color::rgb(0.12, 0.15, 0.10),
            light_text: Color::rgb(0.88, 0.85, 0.90),
            background: Color::rgb(0.79, 0.72, 0.65),
            background_dark: Color::rgb(0.50, 0.45, 0.60),
            accent: Color::rgb(0.69, 0.62, 0.55),
            highlight: Color::rgb(0.07, 0.36, 0.62),
        }
    }

    pub fn light() -> GameColorPalette {
        Self {
            dark_text: Default::default(),
            light_text: Default::default(),
            background: Default::default(),
            background_dark: Default::default(),
            accent: Default::default(),
            highlight: Color::rgb(0.07, 0.36, 0.62),
        }
    }

    pub fn dark_text(&self) -> Color {
        self.dark_text
    }
    pub fn light_text(&self) -> Color {
        self.light_text
    }
    pub fn background(&self) -> Color {
        self.background
    }
    pub fn background_dark(&self) -> Color {
        self.background_dark
    }
    pub fn accent(&self) -> Color {
        self.accent
    }
    pub fn highlight(&self) -> Color {
        self.highlight
    }
}
