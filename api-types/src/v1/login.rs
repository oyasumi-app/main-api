use serde::{Deserialize, Serialize};

use crate::utils::Anonymized;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum LoginRequest {
    EmailPassword {
        email: String,
        password: Anonymized<String>,
    },
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LoginSuccess {
    pub token: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum LoginError {
    /// Either the username/email or the password is incorrect
    /// (not telling which)
    InvalidCredentials,
}
