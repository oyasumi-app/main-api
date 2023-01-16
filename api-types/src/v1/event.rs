use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

pub mod event_type;
pub use event_type::*;

use crate::Snowflake;

use super::EventStreamType;

#[derive(Debug, Serialize, Deserialize)]
pub struct UnparsedEvent {
    pub id: Snowflake,
    pub stream_id: Snowflake,
    pub data: String,
    pub when: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ParsedEvent {
    pub id: Snowflake,
    pub stream_id: Snowflake,
    pub data: Event,
    pub when: DateTime<Utc>,
}

impl ParsedEvent {
    pub fn try_from_unparsed(
        unparsed: &UnparsedEvent,
        event_type: EventStreamType,
    ) -> anyhow::Result<Self> {
        Ok(Self {
            id: unparsed.id,
            stream_id: unparsed.stream_id,
            data: Event::try_parse_as(event_type, &unparsed.data)?,
            when: unparsed.when,
        })
    }
}

impl From<ParsedEvent> for UnparsedEvent {
    fn from(parsed: ParsedEvent) -> Self {
        Self {
            id: parsed.id,
            stream_id: parsed.stream_id,
            data: parsed.data.serialize_into_data(),
            when: parsed.when,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EventCreateRequest {
    // Event stream ID is provided in the path
    pub data: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum EventCreateResponse {
    /// Event created successfully
    Ok,
    /// Event stream does not exist
    StreamNotFound,
    /// Event stream's type cannot parse the given data.
    /// The current event stream type is included.
    /// The error message included comes from the event type's `try_parse_as` function.
    IncompatibleDataType {
        expected_event_type: EventStreamType,
        err: String,
    },
}

#[cfg(feature = "axum_integration")]
impl axum::response::IntoResponse for EventCreateResponse {
    fn into_response(self) -> axum::response::Response {
        match self {
            EventCreateResponse::Ok => (axum::http::StatusCode::CREATED).into_response(),
            EventCreateResponse::StreamNotFound => {
                (axum::http::StatusCode::NOT_FOUND, axum::Json(self)).into_response()
            }
            EventCreateResponse::IncompatibleDataType { .. } => {
                (axum::http::StatusCode::BAD_REQUEST, axum::Json(self)).into_response()
            }
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EventListResponse {
    pub total_count: u64,
    pub events: Vec<ParsedEvent>,
}
