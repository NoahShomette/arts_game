use general::game_meta::{GameId, GamePlayers};

pub fn insert_games_meta_row(game_id: GameId, max_players: u8) -> (String, Vec<String>) {
    let game_id = game_id.to_json();

    (
        "insert into games_meta (game_id, game_players, max_players, game_state, has_space, pending_players) values (?1, ?2, ?3, ?4, ?5, ?6)".to_string(),
        vec![
            game_id,
            serde_json::to_string(&GamePlayers::new())
            .unwrap(),
            max_players.to_string(),
            0.to_string(),
            1.to_string(),
            0.to_string(),
        ],
    )
}
