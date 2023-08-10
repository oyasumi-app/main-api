use std::time::SystemTime;

use api_types::{
    v1::{DateTimeUtc, SleepState},
    Snowflake,
};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use sqlx::query;

use crate::{v1::ResultResponse, AppState, RequireUser};

pub async fn put_by_id(
    State(app_state): State<AppState>,
    RequireUser((conn_user, _conn_token)): RequireUser,
    Path(id): Path<Snowflake>,
    Json(new_state): Json<SleepState>,
) -> ResultResponse<StatusCode> {
    if new_state.id != id {
        return Ok(StatusCode::CONFLICT);
    }
    let start = new_state.start.timestamp();
    let end = new_state.end.map(|i| i.timestamp());
    let row = query!(
        r#"
            UPDATE sleep_state SET
                started_at_unix_time=?,
                ended_at_unix_time=?,
                comment=?
            WHERE user_id=? AND id=?
            RETURNING *"#,
        start,
        end,
        new_state.comment,
        conn_user.id,
        id
    )
    .fetch_optional(&app_state.db)
    .await?;

    match row {
        None => Ok(StatusCode::NOT_FOUND),
        Some(row) => {
            if row.id == Some(id.into()) {
                Ok(StatusCode::NO_CONTENT)
            } else {
                Ok(StatusCode::NOT_FOUND)
            }
        }
    }
}

pub async fn set_current_end(
    State(app_state): State<AppState>,
    RequireUser((conn_user, _conn_token)): RequireUser,
) -> ResultResponse<StatusCode> {
    let now = DateTimeUtc::from(SystemTime::now()).timestamp();
    let row = query!(
        r#"UPDATE sleep_state
        SET ended_at_unix_time=?
        WHERE user_id=? AND ended_at_unix_time IS NULL
        RETURNING sleep_state.id"#,
        now,
        conn_user.id,
    )
    .fetch_optional(&app_state.db)
    .await?;
    match row {
        Some(_row) => Ok(StatusCode::OK),
        None => Ok(StatusCode::NOT_FOUND),
    }
}

pub async fn set_current_start(
    State(app_state): State<AppState>,
    RequireUser((conn_user, _conn_token)): RequireUser,
) -> ResultResponse<StatusCode> {
    let now = DateTimeUtc::from(SystemTime::now()).timestamp();
    let row = query!(
        r#"UPDATE sleep_state
        SET started_at_unix_time=?
        WHERE user_id=? AND ended_at_unix_time IS NULL
        RETURNING sleep_state.id"#,
        now,
        conn_user.id,
    )
    .fetch_optional(&app_state.db)
    .await?;
    match row {
        Some(_row) => Ok(StatusCode::NO_CONTENT),
        None => Ok(StatusCode::NOT_FOUND),
    }
}
