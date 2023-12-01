use bevy::ecs::world::World;
use general::game_meta::NewGameSettings;
/// Function responsible for generating the new game world with the correct setup
pub fn create_game_world(settings: &NewGameSettings) -> World {
    World::new()
}
