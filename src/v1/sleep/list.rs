use api_types::v1::SleepState;
use axum::{extract::State, Json};
use sqlx::query;

use crate::{datetime_utc_from_timestamp, v1::ResultResponse, AppState, RequireUser};

pub async fn list_states(
    State(app_state): State<AppState>,
    RequireUser((conn_user, _conn_token)): RequireUser,
) -> ResultResponse<Json<Vec<SleepState>>> {
    let rows = query!("SELECT * FROM sleep_state WHERE user_id=?", conn_user.id)
        .fetch_all(&app_state.db)
        .await?;

    Ok(Json(
        rows.iter()
            .map(|row| SleepState {
                id: row.id.into(),
                start: datetime_utc_from_timestamp(row.started_at_unix_time),
                end: row.ended_at_unix_time.map(datetime_utc_from_timestamp),
                comment: row.comment.clone(),
            })
            .collect(),
    ))
}
