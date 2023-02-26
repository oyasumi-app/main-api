use serde::{Deserialize, Serialize};

use crate::Snowflake;

#[derive(Debug, Serialize, Deserialize)]
pub struct RegistrationRequest {
    pub username: String,
    pub email: lettre::Address,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "status")]
pub enum RegistrationResponse {
    Ok { id: Snowflake },
    PendingRegistrationExists { id: Snowflake },
    //Err, // TODO: a registration request for an existing username/email MUST NOT be an error (to prevent enumeration attacks)
    DatabaseError,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "status")]
pub enum PendingRegistrationDataResponse {
    /// The pending registration with this ID does not exist.
    /// Either it never did, or it has expired, or it has been already completed and promoted to a user.
    DoesNotExist,
    /// The pending registration exists, and here is its state.
    Exists(PendingRegistration),

    DatabaseError,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PendingRegistration {
    pub username: String,
    pub email: lettre::Address,
    pub can_resend_email_after: chrono::DateTime<chrono::Utc>,
    pub expires_after: chrono::DateTime<chrono::Utc>,
}
