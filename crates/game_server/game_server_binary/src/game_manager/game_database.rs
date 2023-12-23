//! Responsible for saving games and keeping them backed up into the database

use bevy::{
    app::{Plugin, Update},
    ecs::{schedule::IntoSystemConfigs, system::Query},
};
use core_library::sqlite_database::{saving::SaveSchedule, DatabasePlugin};

use crate::app::app_scheduling::ServerAuthenticatedSets;

use super::GameInstance;

pub struct GameDatabasePlugin;

impl Plugin for GameDatabasePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugins(DatabasePlugin {
            database_path: "databases/game_server_database.db".to_string(),
        });

        app.add_systems(
            Update,
            save_games.in_set(ServerAuthenticatedSets::ServerTasks),
        );
    }
}

fn save_games(mut games: Query<&mut GameInstance>) {
    for mut game in games.iter_mut() {
        game.game_world.run_schedule(SaveSchedule);
    }
}
