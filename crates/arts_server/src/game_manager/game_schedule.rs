use bevy::ecs::schedule::ScheduleLabel;

/// A Schedule that is run for each [`GameInstance`].game_world when that game needs to simulate and set its world
#[derive(ScheduleLabel, Hash, Debug, Eq, Clone, PartialEq)]
pub struct GameWorldSimulationSchedule;
