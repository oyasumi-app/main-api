use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};

use crate::{entity::user_token, AppState, RequireUser, Snowflake};

use api_types::v1::token_info::*;

pub async fn get_token(
    State(app_state): State<AppState>,
    RequireUser((conn_user, _conn_token)): RequireUser,
    Path(id): Path<Snowflake>,
) -> Result<Json<TokenData>, StatusCode> {
    // Find the token in the database
    // If the token is not found, return 404
    let (token, user) = match user_token::find_token_by_id(&app_state.db, id).await {
        Ok(Some(token)) => token,
        Ok(None) => return Err(StatusCode::NOT_FOUND),
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };

    // If the token is not owned by the user, return 404
    // (Do not reveal that the token exists)
    if token.user != conn_user.id {
        return Err(StatusCode::NOT_FOUND);
    }

    Ok(Json(TokenData {
        user: TokenUserData {
            id: user.id,
            username: user.username,
            email: user.email,
        },
        token: TokenDetails {
            id: token.id,
            expires: token.expires,
        },
    }))
}

pub async fn get_current_token(
    State(app_state): State<AppState>,
    RequireUser((conn_user, conn_token)): RequireUser,
) -> Result<Json<TokenData>, StatusCode> {
    let id = conn_token.id;
    get_token(
        State(app_state),
        RequireUser((conn_user, conn_token)),
        Path(id),
    )
    .await
}

pub async fn get_token_by_token(
    State(app_state): State<AppState>,
    // no RequireUser here, because the token is provided manually
    Path(token): Path<String>,
) -> Result<Json<TokenData>, StatusCode> {
    // Find the token in the database
    // If the token is not found, return 404
    let (token, user) = match user_token::find_token(&app_state.db, &token).await {
        Ok(Some(token)) => token,
        Ok(None) => return Err(StatusCode::NOT_FOUND),
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };

    Ok(Json(TokenData {
        user: TokenUserData {
            id: user.id,
            username: user.username,
            email: user.email,
        },
        token: TokenDetails {
            id: token.id,
            expires: token.expires,
        },
    }))
}
