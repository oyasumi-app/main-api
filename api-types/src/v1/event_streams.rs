pub mod event_stream_type;

use crate::Snowflake;
use serde::{Deserialize, Serialize};
pub use event_stream_type::EventStreamType;


#[derive(Debug, Serialize, Deserialize)]
pub struct EventStream {
    pub id: Snowflake,
    pub owner: Snowflake,
    pub name: String,
    
    #[serde(rename = "type")]
    pub stream_type: EventStreamType,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct EventStreamPatchRequest {
    #[serde(default)]
    pub name: Option<String>,
    #[serde(rename = "type", default)]
    pub stream_type: Option<EventStreamType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EventStreamCreateRequest {
    pub name: String,
    #[serde(rename = "type")]
    pub stream_type: EventStreamType,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum EventStreamPatchResponse {
    /// The event stream was edited successfully.
    Ok,
    /// The event stream already has events, and they are not compatible with the new type.
    IncompatibleEventsFound,
}

#[cfg(feature="axum_integration")]
impl axum::response::IntoResponse for EventStreamPatchResponse {
    fn into_response(self) -> axum::response::Response {
        match self {
            EventStreamPatchResponse::Ok => {
                (axum::http::StatusCode::OK, axum::Json(self)).into_response()
            }
            EventStreamPatchResponse::IncompatibleEventsFound => {
                (axum::http::StatusCode::BAD_REQUEST, axum::Json(self)).into_response()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_event_stream_type_display() {
        assert_eq!(EventStreamType::StateTransition.to_string(), "StateTransition");
    }
}