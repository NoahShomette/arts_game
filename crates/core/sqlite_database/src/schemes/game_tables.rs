//! Schemas for all the game specific tables

use bevy_state_curves::prelude::SteppedCurve;
use general::{
    game_meta::GameId,
    objects::core_components::{ObjectColor, ObjectGeneral, ObjectId, ObjectPosition},
};

/// Creates a game players table for the given game
pub fn create_game_players(game_id: GameId) -> (String, Vec<String>) {
    let game_id = game_id.id_as_string();

    (
        format!("CREATE TABLE game_players_{} (account_id TEXT PRIMARY KEY NOT NULL, last_sign_in TEXT, last_state_sent TEXT, last_sign_out TEXT, faction TEXT)", game_id),
        vec![
        ],
    )
}
/// Creates a game curves table for the given game
pub fn create_game_curves(game_id: GameId) -> (String, Vec<String>) {
    let game_id = game_id.id_as_string();

    (
        format!("CREATE TABLE game_curves_{} (object_id TEXT PRIMARY KEY NOT NULL, object_general TEXT NOT NULL, sc_object_position TEXT NOT NULL, sc_object_color TEXT NOT NULL)", game_id),
        vec![
        ],
    )
}

/// inserts a new game_curves row
pub fn insert_game_curves_row(
    game_id: GameId,
    object_id: ObjectId,
    object_general: Option<ObjectGeneral>,
    object_position: SteppedCurve<ObjectPosition>,
    object_color: SteppedCurve<ObjectColor>,
) -> Option<(String, Vec<String>)> {
    let game_id = game_id.id_as_string();

    let Ok(object_id) = serde_json::to_string(&object_id) else {
        return None;
    };
    let Ok(object_general) = serde_json::to_string(&object_general) else {
        return None;
    };
    let Ok(object_position) = serde_json::to_string(&object_position) else {
        return None;
    };
    let Ok(object_color) = serde_json::to_string(&object_color) else {
        return None;
    };

    Some((
    format!("insert into game_curves_{} (object_id, object_general, sc_object_position, sc_object_color) values (?1, ?2, ?3, ?4)", game_id),
    vec![
        object_id,
        object_general,
        object_position,
        object_color,
    ],))
}
