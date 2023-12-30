use std::marker::PhantomData;

use bevy::{
    app::Plugin,
    ecs::{
        component::Component,
        query::With,
        system::{Local, Query, ResMut, Resource, SystemId},
    },
    math::Vec2,
    reflect::TypePath,
    ui::UiScale,
    utils::HashMap,
    window::{PrimaryWindow, Window},
};
use bevy_simple_text_input::TextInputPlugin;

use self::{colors::GameColorsPlugin, scenes::UiScenesPlugin, widgets::WidgetsPlugin};

pub mod colors;
pub mod scenes;
pub mod widgets;

pub struct ClientUiPlugin;

impl Plugin for ClientUiPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.init_resource::<UiSystemIdResource>();
        app.add_plugins(TextInputPlugin);
        app.add_plugins((WidgetsPlugin, UiScenesPlugin, GameColorsPlugin));
        //app.add_systems(Update, scale);
    }
}

pub const UI_SCREEN_LAYER: i32 = 1;
pub const UI_MODAL_LAYER: i32 = 100;

#[derive(Resource, Default)]
pub struct UiSystemIdResource {
    pub map: HashMap<&'static str, SystemId>,
}

/// Scales ui to match the screen
pub fn scale(
    mut cached_size: Local<Vec2>,
    mut ui_scale: ResMut<UiScale>,
    windows: Query<&Window, With<PrimaryWindow>>,
) {
    let Some(primary) = windows.iter().next() else {
        return;
    };
    let ww = primary.width();
    let wh = primary.height();
    if cached_size.x == ww && cached_size.y == wh {
        return;
    }
    cached_size.x = ww;
    cached_size.y = wh;

    let scale_h = ww / 1920.0;
    let scale_w = wh / 1080.0;
    ui_scale.0 = scale_h.min(scale_w) as f64;
}

/// Helper function to make creating [`MarkerComponent`] function type helpers easier
pub fn marker_component<Marker: Component + TypePath>() -> MarkerComponent<Marker> {
    MarkerComponent::<Marker> { pd: PhantomData }
}

pub struct MarkerComponent<C: Component + TypePath> {
    pd: PhantomData<C>,
}
