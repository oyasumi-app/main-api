use axum::{
    extract::{Path, State},
    Json,
};

use crate::{
    v1::{ApiError, ResultResponse},
    AppState, RequireUser, Snowflake,
};

use api_types::v1::{event::*, EventStreamType};

#[axum_macros::debug_handler]
pub async fn get_event(
    State(app_state): State<AppState>,
    RequireUser((conn_user, _conn_token)): RequireUser,
    Path((event_stream_id, event_id)): Path<(Snowflake, Snowflake)>,
) -> ResultResponse<Json<ParsedEvent>> {
    // Try finding an Event and EventStream by the given IDs
    let (event, event_stream) =
        match database::entity::event::find_with_stream(&app_state.db, event_stream_id, event_id)
            .await?
        {
            Some(items) => items,
            None => return Err(ApiError::NotFound)?,
        };

    // Check that the event_stream's owner matches the connecting user
    if event_stream.owner != conn_user.id {
        return Err(ApiError::Forbidden)?;
    }

    let stream_type = event_stream.stream_type;
    let stream_type: EventStreamType = match stream_type.parse() {
        Ok(stream_type) => stream_type,
        Err(_) => {
            return Err(ApiError::DatabaseErr(sea_orm::DbErr::Custom(
                "Unexpected stream_type encountered in event_stream record".to_string(),
            )))?
        }
    };

    let unparsed_event: UnparsedEvent = event.into();
    let parsed_event = ParsedEvent::try_from_unparsed(unparsed_event, stream_type)?;

    Ok(Json(parsed_event))
}
