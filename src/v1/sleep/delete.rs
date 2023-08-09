use api_types::Snowflake;
use axum::{
    extract::{Path, State},
    http::StatusCode,
};
use sqlx::query;

use crate::{v1::ResultResponse, AppState, RequireUser};

pub async fn delete_by_id(
    State(app_state): State<AppState>,
    RequireUser((conn_user, _conn_token)): RequireUser,
    Path(id): Path<Snowflake>,
) -> ResultResponse<StatusCode> {
    let row = query!(
        "DELETE FROM sleep_state WHERE user_id=? AND id=? RETURNING sleep_state.id",
        conn_user.id,
        id
    )
    .fetch_optional(&app_state.db)
    .await?;

    match row {
        Some(row) => match row.id {
            Some(_id) => Ok(StatusCode::NO_CONTENT),
            None => Err(crate::v1::ApiError::NotFound)?,
        },
        None => Err(crate::v1::ApiError::NotFound)?,
    }
}

pub async fn delete_current(
    State(app_state): State<AppState>,
    RequireUser((conn_user, _conn_token)): RequireUser,
) -> ResultResponse<StatusCode> {
    let row = query!(
        "DELETE FROM sleep_state WHERE user_id=? AND ended_at_unix_time IS NULL RETURNING sleep_state.id",
        conn_user.id,
    )
    .fetch_optional(&app_state.db)
    .await?;

    match row {
        Some(row) => match row.id {
            Some(_id) => Ok(StatusCode::NO_CONTENT),
            None => Err(crate::v1::ApiError::NotFound)?,
        },
        None => Err(crate::v1::ApiError::NotFound)?,
    }
}
