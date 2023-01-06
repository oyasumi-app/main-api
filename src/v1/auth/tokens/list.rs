use axum::{extract::State, http::StatusCode, Json};

use crate::{entity::user_token, AppState, RequireUser, Snowflake};

pub async fn get_user_tokens(
    State(app_state): State<AppState>,
    RequireUser((conn_user, _conn_token)): RequireUser,
) -> Json<Vec<Snowflake>> {
    Json(
        user_token::find_tokens_by_user(&app_state.db, conn_user.id)
            .await
            .unwrap_or_default()
            .into_iter()
            .map(|token| token.id)
            .collect(),
    )
}

pub async fn delete_user_tokens(
    State(app_state): State<AppState>,
    RequireUser((conn_user, conn_token)): RequireUser,
) -> Result<StatusCode, StatusCode> {
    match user_token::delete_tokens_by_user_except(&app_state.db, conn_user.id, conn_token.id).await
    {
        Ok(_) => Ok(StatusCode::OK),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}
