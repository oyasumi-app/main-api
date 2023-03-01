use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ConfirmRegistrationRequest {
    pub token: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "status")]
pub enum ConfirmRegistrationResponse {
    Ok {
        token: String,
    },
    DatabaseError,
    /// This error means that the registration request does not exist or the token is incorrect.
    RegistrationConfirmError,

    /// This error means that, when we confirmed the registration, a user with
    /// the same username or email already exists.
    /// This should not have happened, because we checked for this before creating the registration request.
    /// Nevertheless, direct the client to restart the registration process.
    UserAlreadyExists,
}
