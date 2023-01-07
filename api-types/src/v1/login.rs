#[derive(Debug, serde::Deserialize)]
#[serde(untagged)]
pub enum LoginRequest {
    UsernamePassword { username: String, password: String },
    EmailPassword { email: String, password: String },
}

#[derive(Debug, serde::Serialize)]
pub enum LoginResponse {
    Ok { token: String },
    Err(LoginError),
}

#[derive(Debug, serde::Serialize)]
pub enum LoginError {
    /// Either the username/email or the password is incorrect
    /// (not telling which)
    InvalidCredentials,
}
