use axum::{extract::State, http::StatusCode, Json};
use sqlx::query;

use crate::{v1::ResultResponse, AppState, RequireUser, Snowflake};

pub async fn get_user_tokens(
    State(app_state): State<AppState>,
    RequireUser((conn_user, _conn_token)): RequireUser,
) -> ResultResponse<Json<Vec<Snowflake>>> {
    let now = chrono::Utc::now().timestamp();
    let token_ids = query!(
        "SELECT id FROM user_token WHERE user_id=? AND expires_unix_time > ?",
        conn_user.id,
        now
    )
    .fetch_all(&app_state.db)
    .await?
    .iter()
    .map(|row| row.id.into())
    .collect();
    Ok(Json(token_ids))
}

pub async fn delete_user_tokens(
    State(app_state): State<AppState>,
    RequireUser((conn_user, conn_token)): RequireUser,
) -> ResultResponse<StatusCode> {
    query!(
        "DELETE FROM user_token WHERE user_id=? AND id!=?",
        conn_user.id,
        conn_token.id
    )
    .execute(&app_state.db)
    .await?;
    Ok(StatusCode::NO_CONTENT)
}
