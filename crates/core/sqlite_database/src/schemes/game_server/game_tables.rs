//! Schemas for all the game specific tables

use bevy::ecs::component::Component;
use bevy_state_curves::prelude::SteppedCurve;
use general::{
    game_meta::GameId,
    objects::core_components::{ObjectGeneral, ObjectId, ObjectPosition},
};

use crate::database_traits::{DatabaseData, DatabaseSql, PureDatabaseData};

/// Creates a new Game Players Table
#[derive(Component, Debug, Clone)]
pub struct CreateGamePlayersTable {
    pub game_id: GameId,
}

impl DatabaseSql for CreateGamePlayersTable {
    fn to_sql(&self) -> Option<(String, Vec<String>)> {
        let game_id = self.game_id.id_as_string();

        Some((
            format!("CREATE TABLE \"game_players_{}\" (account_id TEXT PRIMARY KEY NOT NULL, last_sign_in TEXT, last_state_sent TEXT, last_sign_out TEXT, faction TEXT, color TEXT)", game_id),
            vec![
            ],
        ))
    }
}

/// Creates a new Game Curves Table
#[derive(Component, Debug, Clone)]
pub struct CreateGameCurvesTable {
    pub game_id: GameId,
}

impl DatabaseSql for CreateGameCurvesTable {
    fn to_sql(&self) -> Option<(String, Vec<String>)> {
        let game_id = self.game_id.id_as_string();
        Some(    (
            format!("CREATE TABLE \"game_curves_{}\" (object_id TEXT PRIMARY KEY NOT NULL, sc_object_general TEXT NOT NULL, sc_object_position TEXT NOT NULL)", game_id),
            vec![
            ],
        ))
    }
}

/// Inserts a new GameCurves Row
#[derive(Component, Debug, Clone)]
pub struct InsertGameCurvesRow {
    game_id: GameId,
    object_id: PureDatabaseData,
    object_general: Option<PureDatabaseData>,
    object_position: PureDatabaseData,
}

impl InsertGameCurvesRow {
    pub fn new_row(
        game_id: GameId,
        object_id: &ObjectId,
        object_general: Option<ObjectGeneral>,
        object_position: &SteppedCurve<ObjectPosition>,
    ) -> Option<InsertGameCurvesRow> {
        let Some(object_id) = object_id.to_database_data() else {
            return None;
        };

        let Some(object_position) = object_position.to_database_data() else {
            return None;
        };

        let object_general = match object_general {
            Some(og) => og.to_database_data(),
            None => None,
        };

        Some(InsertGameCurvesRow {
            game_id,
            object_id,
            object_general,
            object_position,
        })
    }
}

impl DatabaseSql for InsertGameCurvesRow {
    fn to_sql(&self) -> Option<(String, Vec<String>)> {
        let game_id = self.game_id.id_as_string();

        match &self.object_general {
        Some(object_general) => {
                Some((format!("insert into \"game_curves_{}\" (object_id, sc_object_general, sc_object_position) values (?1, ?2, ?3)", game_id),
            vec![
                self.object_id.data.clone(),
                object_general.data.clone(),
                self.object_position.data.clone(),
                ],))
            }
            None => Some((format!("insert into \"game_curves_{}\" (object_id, sc_object_general, sc_object_position) values (?1, ?2, ?3)", game_id),
            vec![
                self.object_id.data.clone(),
                ObjectGeneral::default().to_database_data().unwrap().data,
                self.object_position.data.clone(),
            ],)),
        }
    }
}
