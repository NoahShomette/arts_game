use bevy::{
    app::{Plugin, Update},
    ecs::{
        component::Component,
        query::With,
        system::{Local, Query, ResMut},
    },
    math::Vec2,
    ui::UiScale,
    window::{PrimaryWindow, Window},
};

use self::modal::ModalPlugin;

pub mod modal;

pub struct ClientUiPlugin;

impl Plugin for ClientUiPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugins(ModalPlugin);
        app.add_systems(Update, scale);
    }
}

#[derive(Component)]
pub struct DisabledButton;

#[derive(Component)]
pub struct SelectedButton;

#[derive(Component)]
pub struct BasicButton;

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
