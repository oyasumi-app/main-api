use api_types::{v1::SleepState, Snowflake};
use axum::{
    extract::{Path, State},
    Json,
};
use sqlx::query;

use crate::{datetime_utc_from_timestamp, v1::ResultResponse, AppState, RequireUser};

pub async fn get_by_id(
    State(app_state): State<AppState>,
    RequireUser((conn_user, _conn_token)): RequireUser,
    Path(id): Path<Snowflake>,
) -> ResultResponse<Json<SleepState>> {
    let row = query!(
        "SELECT * FROM sleep_state WHERE user_id=? AND id=?",
        conn_user.id,
        id
    )
    .fetch_optional(&app_state.db)
    .await?;

    match row {
        Some(row) => Ok(Json(SleepState {
            id: row.id.into(),
            start: datetime_utc_from_timestamp(row.started_at_unix_time),
            end: row.ended_at_unix_time.map(datetime_utc_from_timestamp),
            comment: row.comment,
        })),
        None => Err(crate::v1::ApiError::NotFound)?,
    }
}

pub async fn get_current(
    State(app_state): State<AppState>,
    RequireUser((conn_user, _conn_token)): RequireUser,
) -> ResultResponse<Json<SleepState>> {
    let row = query!(
        "SELECT * FROM sleep_state WHERE user_id=? AND ended_at_unix_time IS NULL",
        conn_user.id,
    )
    .fetch_optional(&app_state.db)
    .await?;

    match row {
        Some(row) => Ok(Json(SleepState {
            id: row.id.into(),
            start: datetime_utc_from_timestamp(row.started_at_unix_time),
            end: row.ended_at_unix_time.map(datetime_utc_from_timestamp),
            comment: row.comment,
        })),
        None => Err(crate::v1::ApiError::NotFound)?,
    }
}
