use serde::{Deserialize, Serialize};
use strum::{Display, EnumString};

#[derive(Debug, Serialize, Deserialize, EnumString, Display, Clone, Copy, PartialEq, Eq)]
pub enum EventStreamType {
    /// A stream whose events are state transitions.
    /// During the time between events, a particular variable is in the state that the last event specified.
    ///
    /// This is the default type.
    /// This event stream type is used to represent the sleeping/awake state,
    /// which is the main purpose of the app.
    StateTransition,
}
