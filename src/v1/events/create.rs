use axum::{
    extract::{Path, State},
    http::{header, HeaderMap},
    Json,
};

use crate::{
    v1::{ApiError, ResultResponse},
    AppState, RequireUser, Snowflake,
};

use api_types::v1::event::*;

#[axum_macros::debug_handler]
pub async fn create_event(
    State(app_state): State<AppState>,
    RequireUser((conn_user, _conn_token)): RequireUser,
    Path(event_stream_id): Path<Snowflake>,
    Json(request): Json<EventCreateRequest>,
) -> ResultResponse<(HeaderMap, EventCreateResponse)> {
    let mut headers = HeaderMap::new();

    // Try finding an EventStream by this ID
    let maybe_event_stream =
        database::entity::event_stream::find_by_id(&app_state.db, event_stream_id).await?;
    let event_stream = match maybe_event_stream {
        Some(event_stream) => event_stream,
        None => return Ok((headers, EventCreateResponse::StreamNotFound)),
    };

    // Check that the event_stream's owner matches the connecting user
    if event_stream.owner != conn_user.id {
        return Err(ApiError::Forbidden)?;
    }

    // Try parsing the incoming data as the corresponding event type
    let event = Event::try_parse_as(event_stream.stream_type, &request.data);

    // If there was an error parsing it, get the error message and return it
    let event = match event {
        Ok(event) => event,
        Err(err) => {
            return Ok((
                headers,
                EventCreateResponse::IncompatibleDataType {
                    expected_event_type: event_stream.stream_type,
                    err: err.to_string(),
                },
            ))
        }
    };

    // Now that we have a valid event, insert it into the database
    let event_id = database::entity::event::create(
        &app_state.db,
        event_stream.id,
        event.serialize_into_data(),
    )
    .await?;

    // Return the event ID as a Location header
    headers.insert(
        header::LOCATION,
        format!("/v1/events/stream/{event_stream_id}/event/{event_id}")
            .parse()
            .unwrap(),
    );
    Ok((headers, EventCreateResponse::Ok))
}
