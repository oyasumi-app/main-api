use std::collections::HashMap;

use axum::{
    extract::{FromRequestParts, Query, State},
    http::{request::Parts, HeaderMap, Request, StatusCode},
    middleware::Next,
    response::Response,
};

/// Module for dealing with authentication tokens

/// This enum represents the user that is making the request.
/// Either it is the anonymous user (no token),
/// an invalid token,
/// or a valid token and the associated user.
#[derive(Debug, Clone)] // Do not derive serde::Serialize -- this contains sensitive information
pub enum User {
    Anonymous,
    InvalidToken,
    ValidToken(ValidToken),
}

type ValidToken = (crate::entity::user::Model, crate::entity::user_token::Model);

pub async fn auth<B>(
    State(app_state): State<crate::AppState>,
    headers: HeaderMap,
    Query(params): Query<HashMap<String, String>>,
    mut req: Request<B>,
    next: Next<B>,
) -> Result<Response, StatusCode> {
    let mut token = None;
    // Check for a token in the Authorization header
    if let Some(header) = headers.get("Authorization") {
        if let Ok(header) = header.to_str() {
            if let Some(header) = header.strip_prefix("Bearer ") {
                token = Some(header.to_string());
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
        req.extensions_mut().insert(User::Anonymous);
        return Ok(next.run(req).await);
    }

    // Find the token in the database
    let token = token.unwrap();
    let db = &app_state.db;
    let maybe_token = crate::entity::user_token::find_token(db, &token).await;

    if maybe_token.is_err() {
        // Error finding token, return a 500 error
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    }
    let maybe_token = maybe_token.unwrap();

    if maybe_token.is_none() {
        // Token not found, return the invalid token user
        req.extensions_mut().insert(User::InvalidToken);
        return Ok(next.run(req).await);
    } else {
        // Token found, return the valid token user
        let (token, user) = maybe_token.unwrap();
        req.extensions_mut().insert(User::ValidToken((user, token)));
        return Ok(next.run(req).await);
    }
}

/// Extractor to get the user from the request
pub struct ExtractUser(pub User);

#[async_trait::async_trait]
impl<S> FromRequestParts<S> for ExtractUser
where
    S: Send + Sync,
{
    type Rejection = StatusCode;

    async fn from_request_parts(req: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let user = req.extensions.get::<User>().unwrap().clone();
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
        let user = req.extensions.get::<User>().unwrap().clone();
        match user {
            User::Anonymous | User::InvalidToken => Err(StatusCode::UNAUTHORIZED), // TODO: Return a JSON error
            User::ValidToken(token) => Ok(RequireUser(token)),
        }
    }
}
