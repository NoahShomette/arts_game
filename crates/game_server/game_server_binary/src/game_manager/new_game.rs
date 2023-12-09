//! System that generates a new game

use bevy::{
    app::{Plugin, Update},
    ecs::{
        component::Component,
        entity::Entity,
        schedule::IntoSystemConfigs,
        system::{Commands, Query, Res},
        world::{Mut, World},
    },
    hierarchy::DespawnRecursiveExt,
};
use core_library::{
    db_schemes::{
        game_tables::create_game_players, games_meta::insert_games_meta_row, ConnectionSchema,
    },
    game_generation::create_game_world,
    game_meta::NewGameSettings,
    game_meta::{GameId, GamePlayers},
    PendingDatabaseData,
};

use bevy::ecs::system::Command;

use crate::app::app_scheduling::ServerAuthenticatedSets;

use super::{game_database::DatabaseConnection, GameIdMapping, GameInstance};

pub struct NewGamePlugin;

impl Plugin for NewGamePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(
            Update,
            save_new_game_into_database.in_set(ServerAuthenticatedSets::ServerTasks),
        );
    }
}

/// Fn which generates a new game world, spawning it, and setting its initial state to the correct set and then returns the id of the game world's entity
pub fn generate_new_game(
    main_world: &mut World,
    settings: NewGameSettings,
    new_game_id: GameId,
) -> Entity {
    let game_world = create_game_world(&settings);
    let entity = main_world
        .spawn(GameInstance {
            game_id: new_game_id,
            game_world,
            future_actions: vec![],
            game_tick: super::GameTickInfo {
                game_tick: 0,
                ticks_per_tick: settings.ticks_per_tick,
                simulation_tick_amount: settings.simulation_tick_amount,
                last_simulated_tick: 0,
            },
        })
        .id();
    main_world.resource_scope(
        |_world: &mut World, mut game_id_mapping: Mut<GameIdMapping>| {
            game_id_mapping.map.insert(new_game_id.clone(), entity);
        },
    );

    entity
}

/// Command to create a new game
///
/// Must be infallible
pub struct NewGameCommand {
    pub new_game_settings: NewGameSettings,
    pub new_game_id: GameId,
}

impl Command for NewGameCommand {
    fn apply(self, world: &mut bevy::prelude::World) {
        let max_players = self.new_game_settings.max_player_count.clone();
        generate_new_game(world, self.new_game_settings, self.new_game_id);
        world.spawn(PendingDatabaseData {
            data: NewGameMetaTableInfo {
                game_id: self.new_game_id.clone(),
                max_players: max_players,
            },
        });
        world.spawn(PendingDatabaseData {
            data: NewGameInfo {},
        });
    }
}

/// Contains the neccessary information to create all the games required tables
struct NewGameInfo {}

/// Component inserted into the world when a new game is created. Contains the neccessary information to update the "games_meta" table
struct NewGameMetaTableInfo {
    game_id: GameId,
    max_players: u8,
}

/// Saves the game into the games_meta tables as well as creates all the games needed tables
fn save_new_game_into_database(
    database: Res<DatabaseConnection>,
    pending: Query<(Entity, &PendingDatabaseData<NewGameMetaTableInfo>)>,
    mut commands: Commands,
) {
    if pending.is_empty() {
        return;
    }
    if let Ok(mut connection) = database.connection.lock() {
        let Ok(tx) = connection.transaction() else {
            return;
        };
        for (entity, new_game) in pending.iter() {
            let _ = tx.execute_schema(insert_games_meta_row(
                new_game.data.game_id.clone(),
                new_game.data.max_players,
            ));

            let _ = tx.execute_schema(create_game_players(new_game.data.game_id.clone()));

            commands.entity(entity).despawn_recursive();
        }

        let Ok(_) = tx.commit() else {
            return;
        };
    }
}
