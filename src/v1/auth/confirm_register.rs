use axum::{extract::State, Json};
use axum_client_ip::ClientIp;

use crate::{AppState, Snowflake};

#[derive(Debug, serde::Deserialize)]
pub struct ConfirmRegistrationRequest {
    id: Snowflake,
    token: String,
}

#[derive(Debug, serde::Serialize)]
#[serde(tag = "status")]
pub enum ConfirmRegistrationResponse {
    Ok,
    DatabaseError,
    /// This error means that the registration request does not exist or the token is incorrect.
    RegistrationConfirmError,

    /// This error means that, when we confirmed the registration, a user with
    /// the same username or email already exists.
    /// This should not have happened, because we checked for this before creating the registration request.
    /// Nevertheless, direct the client to restart the registration process.
    UserAlreadyExists,
}

#[axum_macros::debug_handler]
pub async fn confirm_register(
    State(app_state): State<AppState>,
    ClientIp(ip): ClientIp,
    Json(request): Json<ConfirmRegistrationRequest>,
) -> Json<ConfirmRegistrationResponse> {
    let pending_registration =
        crate::entity::registration::get_by_id(&app_state.db, request.id).await;
    if pending_registration.is_err() {
        return Json(ConfirmRegistrationResponse::DatabaseError);
    }
    let pending_registration = pending_registration.unwrap();
    if pending_registration.is_none() {
        return Json(ConfirmRegistrationResponse::RegistrationConfirmError);
    }
    let pending_registration = pending_registration.unwrap();
    let token_matches =
        crate::security::token::compare_token(&request.token, &pending_registration.confirm_token);
    if !token_matches {
        return Json(ConfirmRegistrationResponse::RegistrationConfirmError);
    }

    // Now the registration is confirmed.
    // But check if the username/email is already taken.
    // (Normally this should not happen, but if it does, we should not allow the registration.)
    let user =
        crate::entity::user::find_by_username(&app_state.db, &pending_registration.username).await;
    if user.is_err() {
        return Json(ConfirmRegistrationResponse::DatabaseError);
    }
    let user = user.unwrap();
    if user.is_some() {
        return Json(ConfirmRegistrationResponse::UserAlreadyExists);
    }
    let user = crate::entity::user::find_by_email(&app_state.db, &pending_registration.email).await;
    if user.is_err() {
        return Json(ConfirmRegistrationResponse::DatabaseError);
    }
    let user = user.unwrap();
    if user.is_some() {
        return Json(ConfirmRegistrationResponse::UserAlreadyExists);
    }

    // Now we can create the user.
    let user_creation =
        crate::entity::registration::upgrade_to_user(&app_state.db, &pending_registration).await;
    if user_creation.is_err() {
        return Json(ConfirmRegistrationResponse::DatabaseError);
    } else {
        return Json(ConfirmRegistrationResponse::Ok);
    }
}
