use bevy::app::Plugin;
use bevy_asset_loader::loading_state::{LoadingState, LoadingStateAppExt};

use crate::app::AppState;

pub struct AssetLoadingPlugin;

impl Plugin for AssetLoadingPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_loading_state(
            LoadingState::new(AppState::LoadingApp).continue_to_state(AppState::MainMenu),
        );
    }
}
