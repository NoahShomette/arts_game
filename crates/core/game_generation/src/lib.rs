use bevy::{ecs::world::World, math::Vec2, render::color::Color};
use bevy_state_curves::prelude::{CurveTrait, SteppedCurve};
use general::{
    game_meta::NewGameSettings,
    map::outpost::{OutpostColor, OutpostPosition},
};

/// Function responsible for generating the new game world with the correct setup. This *DOES NOT* simulate the world.
/// It simply creates the first set of valid keyframes for the world. The new game must be simulated afterwards before sending it to players
pub fn create_game_world(settings: &NewGameSettings) -> World {
    let mut world = World::new();
    for i in 0..*settings.outpost_count.outpost_count() {
        let mut pos = SteppedCurve::<OutpostPosition>::new();
        let mut color = SteppedCurve::<OutpostColor>::new();

        pos.insert_keyframe(
            0,
            OutpostPosition {
                position: Vec2::new(i as f32, i as f32),
            },
        );
        color.insert_keyframe(
            0,
            OutpostColor {
                color: Color::rgb_u8(i as u8, i as u8, i as u8),
            },
        );

        world.spawn((pos, color));
    }
    world
}
