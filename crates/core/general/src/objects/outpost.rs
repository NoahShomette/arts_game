use bevy_state_curves::prelude::SteppedKeyframe;

/// The other outposts that connect to this outpost
#[derive(Clone)]
pub struct OutpostConnections {}

impl SteppedKeyframe<OutpostConnections> for OutpostConnections {}
