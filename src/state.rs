use parking_lot::Mutex;
use std::fmt::{Display, Formatter, Result as FormatResult};
use std::sync::Arc;

/// State captures the state of a Raft node: Follower, Candidate, Leader, or Shutdown
#[derive(Debug, Copy, Clone)]
pub enum StateType {
    /// Follower is the initial state of a Raft node.
    Follower,

    /// Leader is one of the valid states of a Raft node
    Leader,

    /// Candidate is one of the valid states of a Raft node
    Candidate,

    /// Shutdown is the terminal state of a Raft node
    Shutdown,
}

impl Display for StateType {
    fn fmt(&self, f: &mut Formatter<'_>) -> FormatResult {
        match self {
            StateType::Follower => write!(f, "Follower"),
            StateType::Leader => write!(f, "Leader"),
            StateType::Candidate => write!(f, "Candidate"),
            StateType::Shutdown => write!(f, "Shutdown"),
        }
    }
}

pub struct State {
    /// latest term server has seen
    current_term: u64,

    /// index of highest log entry known to be committed (initialized to 0, increases monotonically)
    commit_index: u64,

    /// index of highest log entry applied to state machine (initialized to 0, increases monotonically)
    last_applied: u64,

    /// cache the latest snapshot index
    last_log_index: u64,

    /// cache the latest snapshot term
    last_log_term: u64,

    /// cache the latest snapshot index
    last_snapshot_index: u64,

    /// cache the latest snapshot term
    last_snapshot_term: u64,

    /// the current state type
    typ: StateType,
}

mod tests {
    use super::*;

    #[test]
    fn test_state() {
        let s = StateType::Follower;

        assert_eq!(format!("{}", s), "Follower");
        assert_eq!(StateType::Candidate.to_string(), "Candidate");
        assert_eq!(StateType::Shutdown.to_string(), "Shutdown");
        assert_eq!(StateType::Leader.to_string(), "Leader");
    }
}