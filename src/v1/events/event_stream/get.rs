use axum::extract::{Json, Path, State};

use crate::v1::ApiError;
use crate::{v1::ResultResponse, AppState, RequireUser};

use crate::Snowflake;
use api_types::v1::event_streams::*;

pub async fn get_event_stream(
    State(app_state): State<AppState>,
    RequireUser((conn_user, _conn_token)): RequireUser,
    Path(id): Path<Snowflake>,
) -> ResultResponse<Json<EventStream>> {
    // Try finding an EventStream by this ID
    let maybe_event_stream = database::entity::event_stream::find_by_id(&app_state.db, id).await?;
    let event_stream = match maybe_event_stream {
        Some(event_stream) => event_stream,
        None => return Err(ApiError::NotFound)?,
    };

    // Check that the event_stream's owner matches the connecting user
    if event_stream.owner != conn_user.id {
        return Err(ApiError::Forbidden)?;
    }

    Ok(Json(event_stream))
}
