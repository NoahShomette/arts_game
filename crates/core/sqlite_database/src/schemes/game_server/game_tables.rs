//! Schemas for all the game specific tables

use bevy::ecs::component::Component;
use bevy_state_curves::prelude::SteppedCurve;
use general::{
    game_meta::GameId,
    objects::core_components::{ObjectColor, ObjectGeneral, ObjectId, ObjectPosition},
};

use crate::database_traits::DatabaseSql;

/// Creates a new Game Players Table
#[derive(Component, Debug)]
pub struct CreateGamePlayersTable {
    pub game_id: GameId,
}

impl DatabaseSql for CreateGamePlayersTable {
    fn to_sql(&self) -> Option<(String, Vec<String>)> {
        let game_id = self.game_id.id_as_string();

        Some((
            format!("CREATE TABLE game_players_{} (account_id TEXT PRIMARY KEY NOT NULL, last_sign_in TEXT, last_state_sent TEXT, last_sign_out TEXT, faction TEXT)", game_id),
            vec![
            ],
        ))
    }
}

/// Creates a new Game Curves Table
#[derive(Component, Debug)]
pub struct CreateGameCurvesTable {
    pub game_id: GameId,
}

impl DatabaseSql for CreateGameCurvesTable {
    fn to_sql(&self) -> Option<(String, Vec<String>)> {
        let game_id = self.game_id.id_as_string();
        Some(    (
            format!("CREATE TABLE game_curves_{} (object_id TEXT PRIMARY KEY NOT NULL, object_general TEXT, sc_object_position TEXT NOT NULL, sc_object_color TEXT NOT NULL)", game_id),
            vec![
            ],
        ))
    }
}

/// Inserts a new GameCurves Row
#[derive(Component, Debug)]
pub struct InsertGameCurvesRow {
    game_id: GameId,
    object_id: ObjectId,
    object_general: Option<ObjectGeneral>,
    object_position: SteppedCurve<ObjectPosition>,
    object_color: SteppedCurve<ObjectColor>,
}

impl DatabaseSql for InsertGameCurvesRow {
    fn to_sql(&self) -> Option<(String, Vec<String>)> {
        let game_id = self.game_id.id_as_string();

        let Ok(object_id) = serde_json::to_string(&self.object_id) else {
            return None;
        };

        let Ok(object_position) = serde_json::to_string(&self.object_position) else {
            return None;
        };
        let Ok(object_color) = serde_json::to_string(&self.object_color) else {
            return None;
        };

        match &self.object_general {
            Some(object_general) => {
                let Ok(object_general) = serde_json::to_string(object_general) else {
                    return None;
                };
                Some((format!("insert into game_curves_{} (object_id, object_general, sc_object_position, sc_object_color) values (?1, ?2, ?3, ?4)", game_id),
            vec![
                object_id,
                object_general,
                object_position,
                object_color,
            ],))
            }
            None => Some((format!("insert into game_curves_{} (object_id, sc_object_position, sc_object_color) values (?1, ?2, ?3)", game_id),
            vec![
                object_id,
                object_position,
                object_color,
            ],)),
        }
    }
}
