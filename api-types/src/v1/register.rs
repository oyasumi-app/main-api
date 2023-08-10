use serde::{Deserialize, Serialize};

use crate::Snowflake;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RegistrationRequest {
    pub username: String,
    pub email: lettre::Address,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "status")]
pub enum RegistrationResponse {
    Ok { id: Snowflake },
    PendingRegistrationExists { id: Snowflake },
    //Err, // TODO: a registration request for an existing username/email MUST NOT be an error (to prevent enumeration attacks)
    DatabaseError,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PendingRegistration {
    pub username: String,
    pub email: lettre::Address,
    pub can_resend_email_after: chrono::DateTime<chrono::Utc>,
    pub expires_after: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "status")]
pub enum ResendConfirmationResponse {
    /// The confirmation was resent.
    /// That is, the mail server has told us that it has sent the message.
    Ok,

    /// There was an error while sending the message, and its string representation is included.
    ///
    /// Note that the request may or may not be retried immediately after this;
    /// check the registration info to make sure.
    SendingError { error: String },

    /// It is too early to resend the message. Check the registration info to know when you should try again.
    TooEarly,
}
