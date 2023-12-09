use bevy::{ecs::world::World, math::Vec2, render::color::Color};
use bevy_state_curves::prelude::{CurveTrait, SteppedCurve};
use general::{
    game_meta::NewGameSettings,
    objects::{ObjectColor, ObjectPosition},
};

/// Function responsible for generating the new game world with the correct setup. This *DOES NOT* simulate the world.
/// It simply creates the first set of valid keyframes for the world. The new game must be simulated afterwards before sending it to players
pub fn create_game_world(settings: &NewGameSettings) -> World {
    let mut world = World::new();
    for i in 0..*settings.map_point_count.map_point_count() {
        let mut pos = SteppedCurve::<ObjectPosition>::new();
        let mut color = SteppedCurve::<ObjectColor>::new();

        pos.insert_keyframe(
            0,
            ObjectPosition {
                position: Vec2::new(i as f32, i as f32),
            },
        );
        color.insert_keyframe(
            0,
            ObjectColor {
                color: Color::rgb_u8(i as u8, i as u8, i as u8),
            },
        );

        world.spawn((pos, color));
    }
    world
}
