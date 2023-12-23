use bevy::ecs::schedule::{Schedule, ScheduleLabel};

#[derive(ScheduleLabel, Hash, PartialEq, Eq, Debug, Clone)]
pub struct GameWorldSimulationSchedule;

impl GameWorldSimulationSchedule {
    #[allow(clippy::let_and_return)]
    pub fn new_schedule() -> Schedule {
        let schedule = Schedule::new(GameWorldSimulationSchedule);
        //schedule.add_systems();

        schedule
    }
}
