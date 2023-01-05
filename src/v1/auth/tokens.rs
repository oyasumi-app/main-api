use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};

use crate::v1::auth::check::{TokenData, TokenDetails, TokenUserData};
use crate::{entity::user_token, AppState, RequireUser, Snowflake};

#[axum::debug_handler]
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


pub async fn delete_token(
    State(app_state): State<AppState>,
    RequireUser((conn_user, _conn_token)): RequireUser,
    Path(id): Path<Snowflake>,
) -> Result<StatusCode, StatusCode> {
    // Find the token in the database
    // If the token is not found, return 404
    let (token, _user) = match user_token::find_token_by_id(&app_state.db, id).await {
        Ok(Some(token)) => token,
        Ok(None) => return Err(StatusCode::NOT_FOUND),
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };

    // If the token is not owned by the user, return 404
    // (Do not reveal that the token exists)
    if token.user != conn_user.id {
        return Err(StatusCode::NOT_FOUND);
    }

    // Delete the token
    match user_token::delete_token(&app_state.db, id).await {
        Ok(_) => Ok(StatusCode::NO_CONTENT),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR), // One error case is if the token is not found, but we already checked for that
    }
}

pub async fn delete_token_by_token(
    State(app_state): State<AppState>,
    // no RequireUser here, because the token is provided manually
    Path(token): Path<String>,
) -> Result<StatusCode, StatusCode> {
    // Find the token in the database
    // If the token is not found, return 404
    let (token, _user) = match user_token::find_token(&app_state.db, &token).await {
        Ok(Some(token)) => token,
        Ok(None) => return Err(StatusCode::NOT_FOUND),
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };

    // Delete the token
    match user_token::delete_token(&app_state.db, token.id).await {
        Ok(_) => Ok(StatusCode::NO_CONTENT),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR), // One error case is if the token is not found, but we already checked for that
    }
}
