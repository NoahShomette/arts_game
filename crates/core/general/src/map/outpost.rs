use bevy::{math::Vec2, render::color::Color};
use bevy_state_curves::prelude::SteppedKeyframe;

/// The Position of an outpost
#[derive(Clone)]
pub struct OutpostPosition {
    pub position: Vec2,
}

impl SteppedKeyframe<OutpostPosition> for OutpostPosition {}

/// The other outposts that connect to this outpost
#[derive(Clone)]
pub struct OutpostConnections {}

impl SteppedKeyframe<OutpostConnections> for OutpostConnections {}

/// A temporary testing keyframe that controls what color the outpost is
#[derive(Clone)]
pub struct OutpostColor {
    pub color: Color,
}

impl SteppedKeyframe<OutpostColor> for OutpostColor {}
