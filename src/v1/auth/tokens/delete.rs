use axum::{
    extract::{Path, State},
    http::StatusCode,
};
use sqlx::query;

use crate::{
    v1::{ApiError, ResultResponse},
    AppState, RequireUser, Snowflake,
};

pub async fn delete_token(
    State(app_state): State<AppState>,
    RequireUser((conn_user, _conn_token)): RequireUser,
    Path(id): Path<Snowflake>,
) -> ResultResponse<StatusCode> {
    // Find the token in the database
    // If the token is not found, return 404
    // If the token is not owned by the user, return 404
    let _token = match query!(
        "SELECT id FROM user_token WHERE id=? AND user_id=?",
        id,
        conn_user.id
    )
    .fetch_optional(&app_state.db)
    .await?
    {
        Some(token_row) => token_row,
        None => return Err(ApiError::NotFound)?,
    };

    // Delete the token
    query!("DELETE FROM user_token WHERE id=?", id)
        .execute(&app_state.db)
        .await?;
    Ok(StatusCode::NO_CONTENT)
}

pub async fn delete_current_token(
    State(app_state): State<AppState>,
    RequireUser((conn_user, conn_token)): RequireUser,
) -> ResultResponse<StatusCode> {
    let id = conn_token.id;
    delete_token(
        State(app_state),
        RequireUser((conn_user, conn_token)),
        Path(id),
    )
    .await
}

pub async fn delete_token_by_token(
    State(app_state): State<AppState>,
    // no RequireUser here, because the token is provided manually
    Path(token): Path<String>,
) -> ResultResponse<StatusCode> {
    // Find the token in the database
    // If the token is not found, return 404
    let token = match query!("SELECT * FROM user_token WHERE token=?", token)
        .fetch_optional(&app_state.db)
        .await?
    {
        Some(token) => token,
        None => return Err(ApiError::NotFound)?,
    };

    // Delete the token
    query!("DELETE FROM user_token WHERE token=?", token.token)
        .execute(&app_state.db)
        .await?;
    Ok(StatusCode::NO_CONTENT)
}
