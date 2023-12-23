use bevy::{
    app::{Plugin, Update},
    ecs::schedule::{
        apply_deferred, common_conditions::in_state, IntoSystemConfigs, IntoSystemSetConfigs,
    },
};
use core_library::authentication::AppAuthenticationState;

use self::app_scheduling::ServerAuthenticatedSets;

pub mod app_scheduling;

pub struct GameAppPlugin;

impl Plugin for GameAppPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.configure_sets(
            Update,
            (
                ServerAuthenticatedSets::ServerTasks,
                ServerAuthenticatedSets::PostServerTasksApplyDeffered,
                ServerAuthenticatedSets::ClientCommunication,
                ServerAuthenticatedSets::GameSimulation,
            )
                .run_if(in_state(AppAuthenticationState::Authenticated))
                .chain(),
        );

        app.add_systems(
            Update,
            apply_deferred.in_set(ServerAuthenticatedSets::PostServerTasksApplyDeffered),
        );
    }
}
