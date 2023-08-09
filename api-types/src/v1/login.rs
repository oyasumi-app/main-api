use serde::{Deserialize, Serialize};

use crate::utils::Anonymized;

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum LoginRequest {
    EmailPassword {
        email: String,
        password: Anonymized<String>,
    },
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginSuccess {
    pub token: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum LoginError {
    /// Either the username/email or the password is incorrect
    /// (not telling which)
    InvalidCredentials,
}
