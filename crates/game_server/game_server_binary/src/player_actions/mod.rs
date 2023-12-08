//! Responsible for simulating and handling [`Actions`] sent by players.

use core_library::{actions::Action, auth_server::AccountId};

/// A player action, is stored by the server and simulated ahead of time but executed only when the tick arrives.
pub struct PlayerAction {
    pub tick_scheduled: u64,
    pub issued_by_player: AccountId,
    pub action: Action,
}
