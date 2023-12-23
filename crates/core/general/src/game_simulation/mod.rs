use bevy::ecs::schedule::{Schedule, ScheduleLabel};

#[derive(ScheduleLabel, Hash, PartialEq, Eq, Debug, Clone)]
pub struct GameWorldSimulationSchedule;

impl GameWorldSimulationSchedule {
    pub fn new() -> Schedule {
        let schedule = Schedule::new(GameWorldSimulationSchedule);
        //schedule.add_systems();

        schedule
    }
}
