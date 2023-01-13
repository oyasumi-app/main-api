use serde::{Deserialize, Serialize};

use crate::Snowflake;

#[derive(Debug, Serialize, Deserialize)]
pub struct RegistrationRequest {
    pub username: String,
    pub email: String,
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
