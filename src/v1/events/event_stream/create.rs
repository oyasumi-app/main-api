use axum::{
    extract::{Json, State},
    http::{HeaderMap, StatusCode},
};

use crate::{v1::ResultResponse, AppState, RequireUser};

use api_types::v1::event_streams::*;

#[axum_macros::debug_handler]
pub async fn new_event_stream(
    State(app_state): State<AppState>,
    RequireUser((conn_user, _conn_token)): RequireUser,
    Json(request): Json<EventStreamCreateRequest>,
) -> ResultResponse<(HeaderMap, StatusCode)> {
    let id = database::entity::event_stream::create(&app_state.db, conn_user.id, request).await?;
    // Set Location header to URL to new event stream
    let mut headers = HeaderMap::new();
    headers.insert(
        "Location",
        format!("/v1/events/stream/{id}").parse().unwrap(),
    );
    Ok((headers, StatusCode::CREATED))
}
