use serde::{Deserialize, Serialize};

use crate::Snowflake;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "status")]
pub enum CheckResponse {
    Anonymous,
    InvalidToken,
    ValidToken(TokenData),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TokenData {
    pub user: TokenUserData,
    pub token: TokenDetails,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TokenUserData {
    pub id: Snowflake,
    pub username: String,
    pub email: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TokenDetails {
    pub id: Snowflake,
    pub expires: chrono::DateTime<chrono::Utc>,
}
