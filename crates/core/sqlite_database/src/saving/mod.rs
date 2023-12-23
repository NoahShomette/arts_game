//! Responsible for automatically saving changed data into the database. Will send an [`AsyncChannelSender`] message for each component that changes.
//!
//! Note that this does not Insert new rows and that this runs in the game world and not the server world

use bevy::{
    app::{App, Plugin},
    ecs::{
        component::Component,
        query::With,
        removal_detection::RemovedComponents,
        schedule::{Schedule, ScheduleLabel},
        system::{Commands, Query, Res, Resource},
    },
};
use bevy_state_curves::prelude::SteppedCurve;
use general::{
    game_meta::GameId,
    objects::core_components::{ObjectGeneral, ObjectId, ObjectPosition},
    AsyncChannelSender,
};

use crate::{
    database_traits::{DatabaseData, GameDatabaseTable},
    schemes::game_server::GameCurvesTable,
    update_row::UpdateRow,
};

pub struct DatabaseSavePlugin;

impl Plugin for DatabaseSavePlugin {
    fn build(&self, _app: &mut App) {}
}

/// A schedule that will check for any components that have changed since the last time it was run and will send
/// commands and messages to save everything that has changed
#[derive(ScheduleLabel, Hash, PartialEq, Eq, Debug, Clone)]
pub struct SaveSchedule;

impl SaveSchedule {
    /// Creates a new Schedule with all the neccesary save schedule systems
    pub fn new() -> Schedule {
        let mut schedule = Schedule::new(SaveSchedule);
        schedule.add_systems((
            save_component::<GameCurvesTable, ObjectId, SteppedCurve<ObjectPosition>>,
            save_component::<GameCurvesTable, ObjectId, ObjectGeneral>,
        ));

        schedule
    }
}

#[derive(Component)]
pub struct ExistsInDatabase;

/// Fn that sends an UpdateRow message for any component that has changed and has an [`ExistsInDatabase`] component. Note that when
fn save_component<
    Table: Resource + GameDatabaseTable,
    RowId: Component + DatabaseData,
    C: Component + DatabaseData,
>(
    mut commands: Commands,
    query: Query<(&RowId, &C), (bevy::prelude::Changed<C>, With<ExistsInDatabase>)>,
    mut removed_components: RemovedComponents<C>,
    update_row_channel: Res<AsyncChannelSender<UpdateRow>>,
    table: Res<Table>,
    game_id: Res<GameId>,
) where
    C: Component + DatabaseData,
{
    for (row_id, c) in query.iter() {
        let Some(row_id) = row_id.to_database_data() else {
            continue;
        };
        let Some(data) = c.to_database_data() else {
            continue;
        };
        let _ = update_row_channel.sender_channel.send(UpdateRow {
            table_name: table.table_name(&game_id),
            row_id: row_id,
            database_data: vec![data],
        });
    }

    for entity in removed_components.read() {
        if let Some(mut _entity_commands) = commands.get_entity(entity) {}
    }
}
