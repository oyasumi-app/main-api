use std::time::SystemTime;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use axum_client_ip::ClientIp;
use database::entity::registration::resend_confirmation_email_by_model;
use sea_orm::prelude::DateTimeUtc;

use crate::{
    v1::{ApiError, ResultResponse},
    AppState,
};

use api_types::{v1::register::*, Snowflake};

#[axum_macros::debug_handler]
pub async fn make_registration(
    State(app_state): State<AppState>,
    ClientIp(ip): ClientIp,
    Json(request): Json<RegistrationRequest>,
) -> Json<RegistrationResponse> {
    let pending_registration = database::entity::registration::find_pending_registration(
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
            // NB: even if the username/email is already taken, we still create a registration request.
            // This is to prevent enumeration attacks.
            // make_registration will still create it,
            // but it will not send the confirmation email;
            // instead, it will send an email to the owner of the existing account.
            // This email will not contain the confirmation token,
            // thus making it impossible to confirm the registration.
            let registration = database::entity::registration::make_registration(
                app_state.db,
                &request.username,
                request.email,
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

#[axum_macros::debug_handler]
pub async fn get_registration(
    State(app_state): State<AppState>,
    Path(registration_id): Path<Snowflake>,
) -> ResultResponse<Json<PendingRegistration>> {
    let pending_registration =
        database::entity::registration::get_by_id(&app_state.db, registration_id).await;
    match pending_registration {
        Ok(None) => Err(ApiError::NotFound.into()),
        Ok(Some(reg)) => {
            let email = match reg.email.parse() {
                Ok(email) => email,
                Err(err) => {
                    tracing::error!(
                        "Error parsing email {:?} from registration {} from database?! {}",
                        reg.email,
                        registration_id,
                        err
                    );
                    return Err(ApiError::UnexpectedError(format!("Could not parse email as lettre::Address in registration {} (email is {:?})", registration_id, reg.email)).into());
                }
            };
            let reg = PendingRegistration {
                username: reg.username,
                email,
                can_resend_email_after: reg.email_resend_after,
                expires_after: reg.expires,
            };
            Ok(Json(reg))
        }
        Err(err) => Err(err.into()),
    }
}

#[axum_macros::debug_handler]
pub async fn resend_confirm_email(
    State(app_state): State<AppState>,
    Path(registration_id): Path<Snowflake>,
) -> ResultResponse<(StatusCode, Json<ResendConfirmationResponse>)> {
    // Get the registration by id, bail early if not found
    let pending_registration =
        database::entity::registration::get_by_id(&app_state.db, registration_id).await;
    let registration = match pending_registration {
        Ok(None) => return Err(ApiError::NotFound.into()),
        Err(err) => return Err(err.into()),
        Ok(Some(reg)) => reg,
    };

    // Check if we are now after the retry time
    let now: DateTimeUtc = SystemTime::now().into();
    if registration.email_resend_after < now {
        return Ok((
            StatusCode::TOO_MANY_REQUESTS,
            Json(ResendConfirmationResponse::TooEarly),
        ));
    }

    // Try resending the email
    match resend_confirmation_email_by_model(&app_state.db, &registration).await {
        Err(db_err) => return Err(db_err.into()),
        Ok(Err(email_err)) => {
            return Ok((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ResendConfirmationResponse::SendingError {
                    error: email_err.to_string(),
                }),
            ))
        }
        Ok(Ok(_)) => return Ok((StatusCode::OK, Json(ResendConfirmationResponse::Ok))),
    };
}
