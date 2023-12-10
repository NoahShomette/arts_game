//! Schemas for all the game specific tables

use general::game_meta::GameId;

pub fn create_game_players(game_id: GameId) -> (String, Vec<String>) {
    let game_id = game_id.id_as_string();

    (
        format!("CREATE TABLE game_players_{} (account_id TEXT PRIMARY KEY NOT NULL, last_sign_in TEXT, last_state_sent TEXT, last_sign_out TEXT, faction TEXT)", game_id),
        vec![
        ],
    )
}

pub fn create_game_curves(game_id: GameId) -> (String, Vec<String>) {
    let game_id = game_id.id_as_string();

    (
        format!("CREATE TABLE game_curves_{} (object_id TEXT PRIMARY KEY NOT NULL, object_general TEXT NOT NULL, sc_object_position TEXT NOT NULL, sc_object_color TEXT NOT NULL)", game_id),
        vec![
        ],
    )
}
