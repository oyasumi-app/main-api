use axum::extract::{Json, State};

use crate::{v1::ResultResponse, AppState, RequireUser};

use api_types::v1::event_streams::*;

#[axum_macros::debug_handler]
pub async fn list_event_streams(
    State(app_state): State<AppState>,
    RequireUser((conn_user, _conn_token)): RequireUser,
) -> ResultResponse<Json<Vec<EventStream>>> {
    Ok(Json(
        database::entity::event_stream::find_by_owner(&app_state.db, conn_user.id).await?,
    ))
}
