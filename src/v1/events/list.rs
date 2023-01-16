use axum::extract::{Json, Path, Query, State};
use serde::Deserialize;

use crate::{
    v1::{ApiError, ResultResponse},
    AppState, RequireUser,
};

use api_types::{
    v1::{EventListResponse, ParsedEvent},
    Snowflake,
};

#[derive(Deserialize)]
pub struct Offset {
    since: Option<Snowflake>,
}

const EVENT_COUNT: u64 = 100;

#[axum_macros::debug_handler]
pub async fn list_events(
    State(app_state): State<AppState>,
    RequireUser((conn_user, _conn_token)): RequireUser,
    Path(id): Path<Snowflake>,
    Query(offset): Query<Offset>,
) -> ResultResponse<Json<EventListResponse>> {
    // Check that the event_stream's owner matches the connecting user
    let event_stream = match database::entity::event_stream::find_by_id(&app_state.db, id).await? {
        Some(event_stream) => event_stream,
        None => return Err(ApiError::NotFound)?,
    };

    let event_stream_type = event_stream.stream_type;

    if event_stream.owner != conn_user.id {
        return Err(ApiError::Forbidden)?;
    }

    let count = database::entity::event::count_by_stream(&app_state.db, id).await?;

    let since = offset.since.unwrap_or_default();

    let events =
        database::entity::event::list_by_stream(&app_state.db, id, EVENT_COUNT, since).await?;
    let unparsed_events = events.into_iter().map(|event| event.into());
    let mut parsed_events = vec![];
    for unparsed_event in unparsed_events {
        let maybe_parsed_event = ParsedEvent::try_from_unparsed(&unparsed_event, event_stream_type);
        match maybe_parsed_event {
            Ok(parsed_event) => parsed_events.push(parsed_event),
            Err(err) => {
                return Err(ApiError::NeverError(
                    format!("Event {unparsed_event:?} could not be parsed as {event_stream_type:?}: {err:?} (in event_stream ID {id}"),
                ))?
            }
        }
    }

    let event_response = EventListResponse {
        total_count: count,
        events: parsed_events,
    };
    Ok(Json(event_response))
}
