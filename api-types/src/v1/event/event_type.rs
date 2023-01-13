use serde::{Deserialize, Serialize};

use crate::v1::EventStreamType;

/// The EventType trait is implemented by types that represent events and event streams.
pub trait EventType {
    /// The data type that is stored in the event.
    type Data: Serialize + for<'a> Deserialize<'a>;

    /// The name of the event type.
    const EVENT_TYPE_NAME: &'static str;

    /// Attempt to parse the given string as the data type for this event type.
    fn try_parse_as(data: &str) -> anyhow::Result<Self>
    where
        Self: Sized;

    /// Convert this event's data to a string.
    /// This is the inverse of `try_parse_as`.
    /// The returned string should be parseable by `try_parse_as`.
    fn serialize_into_data(&self) -> String;
}

/// An event is a single data item that is stored in an event stream.
/// Its type is determined by the event stream's type.
#[derive(Debug, Serialize, Deserialize)]
pub enum Event {
    StateTransition(StateTransition),
}

impl Event {
    /// Attempt to parse the given string as an event of the provided type.
    pub fn try_parse_as(event_type: EventStreamType, data: &str) -> anyhow::Result<Self> {
        match event_type {
            EventStreamType::StateTransition => {
                Ok(Self::StateTransition(StateTransition::try_parse_as(data)?))
            }
        }
    }

    /// Convert this event's data to a string.
    pub fn serialize_into_data(&self) -> String {
        match self {
            Self::StateTransition(event) => event.serialize_into_data(),
        }
    }
}

/// An event representing a state transition.
/// The contained data is a string: the name of the new state.
#[derive(Debug, Serialize, Deserialize)]
pub struct StateTransition(String);

impl EventType for StateTransition {
    const EVENT_TYPE_NAME: &'static str = "StateTransition";

    type Data = String;

    fn try_parse_as(data: &str) -> anyhow::Result<Self> {
        Ok(Self(data.to_owned()))
    }

    fn serialize_into_data(&self) -> String {
        self.0.clone()
    }
}
