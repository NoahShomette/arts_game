use bevy::{math::Vec2, render::color::Color};
use bevy_state_curves::prelude::SteppedKeyframe;

pub mod outpost;

/// The Position of an object
#[derive(Clone)]
pub struct ObjectPosition {
    pub position: Vec2,
}

impl SteppedKeyframe<ObjectPosition> for ObjectPosition {}

/// A temporary testing keyframe that controls what color an object is
#[derive(Clone)]
pub struct ObjectColor {
    pub color: Color,
}

impl SteppedKeyframe<ObjectColor> for ObjectColor {}
