use axum::{extract::State, Json};
use axum_client_ip::ClientIp;

use crate::{AppState, Snowflake};

#[derive(Debug, serde::Deserialize)]
pub struct RegistrationRequest {
    username: String,
    email: String,
    password: String,
}

#[derive(Debug, serde::Serialize)]
#[serde(tag = "status")]
pub enum RegistrationResponse {
    Ok { id: Snowflake },
    PendingRegistrationExists { id: Snowflake },
    //Err, // TODO: a registration request for an existing username/email MUST NOT be an error (to prevent enumeration attacks)
    DatabaseError,
}

#[axum_macros::debug_handler]
pub async fn register(
    State(app_state): State<AppState>,
    ClientIp(ip): ClientIp,
    Json(request): Json<RegistrationRequest>,
) -> Json<RegistrationResponse> {
    let pending_registration = crate::entity::registration::find_pending_registration(
        &app_state.db,
        &request.username,
        &request.email,
    )
    .await;
    match pending_registration {
        Ok(Some(pending_registration)) => Json(RegistrationResponse::PendingRegistrationExists {
            id: pending_registration.id,
        }),
        Ok(None) => {
            let registration = crate::entity::registration::make_registration(
                app_state.db,
                &request.username,
                &request.email,
                &request.password,
                &ip,
            )
            .await;
            match registration {
                Ok(registration_id) => Json(RegistrationResponse::Ok {
                    id: registration_id,
                }),
                Err(_) => Json(RegistrationResponse::DatabaseError),
            }
        }
        Err(_) => Json(RegistrationResponse::DatabaseError),
    }
}
