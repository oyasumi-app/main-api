use std::time::SystemTime;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use sqlx::query;

use crate::{
    datetime_utc_from_timestamp,
    v1::{ApiError, ResultResponse},
    AppState, DateTimeUtc, RequireUser, Snowflake,
};

use api_types::v1::token_info::*;

pub async fn get_token(
    State(app_state): State<AppState>,
    RequireUser((conn_user, _conn_token)): RequireUser,
    Path(id): Path<Snowflake>,
) -> ResultResponse<Json<TokenData>> {
    // Find the token in the database
    // If the token is not found, return 404
    // If the token is not owned by the user, return 404
    let now = DateTimeUtc::from(SystemTime::now()).timestamp();
    let token = match query!(
        "SELECT * FROM user_token WHERE id=? AND user_id=? AND expires_unix_time>?",
        id,
        conn_user.id,
        now
    )
    .fetch_optional(&app_state.db)
    .await?
    {
        Some(token_row) => token_row,
        None => return Err(ApiError::NotFound)?,
    };

    Ok(Json(TokenData {
        user: TokenUserData {
            id: conn_user.id.into(),
            username: conn_user.username,
            email: conn_user.email,
        },
        token: TokenDetails {
            id: token.id.into(),
            expires: datetime_utc_from_timestamp(token.expires_unix_time),
        },
    }))
}

pub async fn get_current_token(
    State(app_state): State<AppState>,
    RequireUser((conn_user, conn_token)): RequireUser,
) -> ResultResponse<Json<TokenData>> {
    let id = conn_token.id;
    get_token(
        State(app_state),
        RequireUser((conn_user, conn_token)),
        Path(id),
    )
    .await
}

#[axum::debug_handler]
pub async fn get_token_by_token(
    State(app_state): State<AppState>,
    // no RequireUser here, because the token is provided manually
    Path(token): Path<String>,
) -> Result<Json<TokenData>, StatusCode> {
    // Find the token in the database
    // If the token is not found, return 404
    let now = DateTimeUtc::from(SystemTime::now()).timestamp();
    let row = match query!(
        r#"SELECT
            user_token.id as "user_token_id: Snowflake",
            user_token.token,
            user_token.user_id as "user_id: Snowflake",
            user_token.created_by_ip,
            user_token.expires_unix_time,
            user.username,
            user.email,
            user.password_hash
         FROM user_token INNER JOIN user ON user.id = user_token.user_id WHERE user_token.token=? AND user_token.expires_unix_time > ?"#,
        token, now
    )
    .fetch_optional(&app_state.db)
    .await.unwrap()
    {
        Some(token_row) => token_row,
        None => return Err(StatusCode::NOT_FOUND),
    };

    Ok(Json(TokenData {
        user: TokenUserData {
            id: row.user_id,
            username: row.username,
            email: row.email,
        },
        token: TokenDetails {
            id: row.user_token_id,
            expires: datetime_utc_from_timestamp(row.expires_unix_time),
        },
    }))
}
