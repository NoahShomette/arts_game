//! Scenes are essentially distinct Ui modes.
//!
//! For now we really just have two and then theres substates in those.
//!
//! - Main Menu
//! - Game

use self::main_menu::UiMainMenuPlugin;
use bevy::{
    app::{App, Plugin},
    ecs::{
        component::Component,
        entity::Entity,
        query::With,
        schedule::{OnEnter, OnExit, States},
        system::{Commands, IntoSystem, Query, Res},
    },
    hierarchy::DespawnRecursiveExt,
    reflect::TypePath,
};

use super::{MarkerComponent, UiSystemIdResource};

pub mod main_menu;

pub struct UiScenesPlugin;

impl Plugin for UiScenesPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugins(UiMainMenuPlugin);
    }
}

pub trait ScenesAppExtension {
    /// Adds a new scene that will run the setup system every time the given state is entered and a cleanup system every time it leaves.
    fn add_scene<Marker: Component + TypePath, M>(
        &mut self,
        marker_component: MarkerComponent<Marker>,
        setup_system: impl IntoSystem<(), (), M> + 'static,
        state: impl States,
    );
}

impl ScenesAppExtension for App {
    fn add_scene<Marker: Component + TypePath, M>(
        &mut self,
        _: MarkerComponent<Marker>,
        setup_system: impl IntoSystem<(), (), M> + 'static,
        states: impl States,
    ) {
        let system_id = self.world.register_system(setup_system);
        let mut resource = self.world.resource_mut::<UiSystemIdResource>();
        resource.map.insert(Marker::type_path(), system_id);
        self.add_systems(OnEnter(states.clone()), setup_scene::<Marker>);
        self.add_systems(OnExit(states), cleanup_scene::<Marker>);
    }
}

fn setup_scene<SceneRootMarker: Component + TypePath>(
    mut commands: Commands,
    resource: Res<UiSystemIdResource>,
) {
    let system_id = resource.map.get(SceneRootMarker::type_path()).unwrap();
    commands.run_system(*system_id);
}

fn cleanup_scene<SceneRootMarker: Component>(
    query: Query<Entity, With<SceneRootMarker>>,
    mut commands: Commands,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
