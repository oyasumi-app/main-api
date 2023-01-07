use serde::{Deserialize, Serialize};

use crate::Snowflake;

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "status")]
pub enum CheckResponse {
    Anonymous,
    InvalidToken,
    ValidToken(TokenData),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenData {
    pub user: TokenUserData,
    pub token: TokenDetails,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenUserData {
    pub id: Snowflake,
    pub username: String,
    pub email: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenDetails {
    pub id: Snowflake,
    pub expires: chrono::DateTime<chrono::Utc>,
}
