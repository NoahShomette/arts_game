use bevy::ecs::schedule::SystemSet;

/// Server set that is run when the server is authenticated and running
#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
pub enum ServerAuthenticatedSets {
    /// Important critical tasks that must be completed before any other tasks are finished
    /// - Updating mappings between PlayerIds, ConnectionIds, GameIds, Etc
    /// - Starting Games
    /// - Saving games
    ServerTasks,
    /// Simple set that runs apply deffered to make sure the app is updated
    PostServerTasksApplyDeffered,
    /// All general non critical client-game communication goes here
    /// - Clients requesting state
    /// - Clients sending Actions
    ///
    ClientCommunication,
    /// Set that runs the app simulation
    GameSimulation,
}
