use api_types::{v1::SleepState, Snowflake};
use axum::{extract::State, http::StatusCode, Json};
use sqlx::query;

use crate::{v1::ResultResponse, AppState, RequireUser};

pub async fn create_now(
    State(app_state): State<AppState>,
    RequireUser((conn_user, _conn_token)): RequireUser,
) -> ResultResponse<Result<(StatusCode, Json<SleepState>), StatusCode>> {
    // Check if there is a row with no end time.
    // If there is, return a Conflict
    let existing_row = query!(
        "SELECT * FROM sleep_state WHERE user_id=? AND ended_at_unix_time IS NULL",
        conn_user.id
    )
    .fetch_optional(&app_state.db)
    .await?;
    if let Some(_row) = existing_row {
        return Ok(Err(StatusCode::CONFLICT));
    }

    let id = Snowflake::new().await;
    let now = id.timestamp().timestamp();
    query!(
        r#"INSERT INTO sleep_state
            (id, user_id, started_at_unix_time, ended_at_unix_time, comment)
            VALUES (?,?,?,?,?)"#,
        id,
        conn_user.id,
        now,
        Option::<i64>::None,
        Option::<String>::None,
    )
    .execute(&app_state.db)
    .await?;

    Ok(Ok((
        StatusCode::CREATED,
        Json(SleepState {
            id,
            start: id.timestamp(),
            end: None,
            comment: None,
        }),
    )))
}
