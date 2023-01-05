use axum::Json;

use crate::{ExtractUser, Snowflake, User};

#[derive(Debug, serde::Serialize)]
#[serde(tag = "status")]
pub enum CheckResponse {
    Anonymous,
    InvalidToken,
    ValidToken(TokenData),
}

#[derive(Debug, serde::Serialize)]
pub struct TokenData {
    pub user: TokenUserData,
    pub token: TokenDetails,
}

#[derive(Debug, serde::Serialize)]
pub struct TokenUserData {
    pub id: Snowflake,
    pub username: String,
    pub email: String,
}

#[derive(Debug, serde::Serialize)]
pub struct TokenDetails {
    pub id: Snowflake,
    pub expires: chrono::DateTime<chrono::Utc>,
}

pub async fn check(ExtractUser(user): ExtractUser) -> Json<CheckResponse> {
    match user {
        User::Anonymous => Json(CheckResponse::Anonymous),
        User::InvalidToken => Json(CheckResponse::InvalidToken),
        User::ValidToken((user, token)) => Json(CheckResponse::ValidToken(TokenData {
            user: TokenUserData {
                id: user.id,
                username: user.username,
                email: user.email,
            },
            token: TokenDetails {
                id: token.id,
                expires: token.expires,
            },
        })),
    }
}
