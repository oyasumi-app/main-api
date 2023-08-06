use std::collections::HashMap;

use api_types::Snowflake;
use axum::{
    extract::{FromRequestParts, Query, State},
    http::{request::Parts, HeaderMap, Request, StatusCode},
    middleware::Next,
    response::Response,
};
use chrono::{DateTime, Utc};
use sqlx::query;

use crate::datetime_utc_from_timestamp;

/// Module for dealing with authentication tokens

/// This enum represents the user that is making the request.
/// Either it is the anonymous user (no token),
/// an invalid token,
/// or a valid token and the associated user.
#[derive(Debug, Clone)] // Do not derive serde::Serialize -- this contains sensitive information
pub enum LoginState {
    Anonymous,
    InvalidToken,
    ValidToken(ValidToken),
}

#[derive(Debug, Clone)]
pub struct User {
    pub id: Snowflake,
    pub username: String,
    pub email: String,
    pub password_hash: String,
}
#[derive(Debug, Clone)]
pub struct UserToken {
    pub id: Snowflake,
    pub token: String,
    pub user_id: Snowflake,
    pub created_by_ip: String,
    pub expires: DateTime<Utc>,
}

type ValidToken = (User, UserToken);

pub async fn auth<B>(
    State(app_state): State<crate::AppState>,
    headers: HeaderMap,
    Query(params): Query<HashMap<String, String>>,
    mut req: Request<B>,
    next: Next<B>,
) -> Result<Response, StatusCode> {
    let mut token = None;
    // Check for a token in the cookie named "Token"
    if let Some(cookie) = headers.get("Cookie") {
        if let Ok(cookie) = cookie.to_str() {
            for cookie in cookie.split(';') {
                let cookie = cookie.trim();
                if let Some(cookie) = cookie.strip_prefix("Token=") {
                    token = Some(cookie.to_string());
                }
            }
        }
    }
    if token.is_none() {
        // Check for a token in the Authorization header
        if let Some(header) = headers.get("Authorization") {
            if let Ok(header) = header.to_str() {
                if let Some(header) = header.strip_prefix("Bearer ") {
                    token = Some(header.to_string());
                }
            }
        }
    }
    if token.is_none() {
        // Check for a token in the query string
        if let Some(query_token) = params.get("token") {
            token = Some(query_token.to_string());
        }
    }

    if token.is_none() {
        // No token was found, return the anonymous user
        req.extensions_mut().insert(LoginState::Anonymous);
        return Ok(next.run(req).await);
    }

    // Find the token in the database
    let token = token.unwrap();
    let db = &app_state.db;
    let row = query!(
        r#"SELECT
            user_token.id as "user_token_id: Snowflake",
            user_token.token,
            user_token.user_id as "user_id: Snowflake",
            user_token.created_by_ip,
            user_token.expires_unix_time,
            user.username,
            user.email,
            user.password_hash
        FROM user_token INNER JOIN user ON user.id = user_token.user_id WHERE token=?"#,
        token
    )
    .fetch_optional(db)
    .await;

    let row = match row {
        Ok(row) => row,
        Err(_e) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };

    if row.is_none() {
        // Token not found, return the invalid token user
        req.extensions_mut().insert(LoginState::InvalidToken);
        return Ok(next.run(req).await);
    } else {
        // Token found, return the valid token user
        let row = row.unwrap();
        let user = User {
            id: row.user_id,
            username: row.username,
            email: row.email,
            password_hash: row.password_hash,
        };
        let token = UserToken {
            id: row.user_token_id,
            token: row.token,
            user_id: row.user_id,
            created_by_ip: row.created_by_ip,
            expires: datetime_utc_from_timestamp(row.expires_unix_time),
        };
        req.extensions_mut()
            .insert(LoginState::ValidToken((user, token)));
        return Ok(next.run(req).await);
    }
}

/// Extractor to get the user from the request
pub struct ExtractUser(pub LoginState);

#[async_trait::async_trait]
impl<S> FromRequestParts<S> for ExtractUser
where
    S: Send + Sync,
{
    type Rejection = StatusCode;

    async fn from_request_parts(req: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let user = req.extensions.get::<LoginState>().unwrap().clone();
        Ok(ExtractUser(user))
    }
}

/// Extractor that extracts a user, and returns a 401 error if the user is anonymous or invalid
pub struct RequireUser(pub ValidToken);

#[async_trait::async_trait]
impl<S> FromRequestParts<S> for RequireUser
where
    S: Send + Sync,
{
    type Rejection = StatusCode;

    async fn from_request_parts(req: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let user = req.extensions.get::<LoginState>().unwrap().clone();
        match user {
            LoginState::Anonymous | LoginState::InvalidToken => Err(StatusCode::UNAUTHORIZED), // TODO: Return a JSON error
            LoginState::ValidToken(token) => Ok(RequireUser(token)),
        }
    }
}
