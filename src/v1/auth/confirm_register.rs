use axum::{
    extract::{Path, State},
    http::{HeaderMap, StatusCode},
    Json,
};

use crate::AppState;

use api_types::{v1::confirm_register::*, Snowflake};

#[axum_macros::debug_handler]
pub async fn confirm_registration(
    State(app_state): State<AppState>,
    Path(reg_id): Path<Snowflake>,
    Json(request): Json<ConfirmRegistrationRequest>,
) -> (StatusCode, HeaderMap, Json<ConfirmRegistrationResponse>) {
    let mut headers = HeaderMap::new();
    let pending_registration =
        database::entity::registration::get_by_id(&app_state.db, reg_id).await;
    if pending_registration.is_err() {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            headers,
            Json(ConfirmRegistrationResponse::DatabaseError),
        );
    }
    let pending_registration = pending_registration.unwrap();
    if pending_registration.is_none() {
        return (
            StatusCode::BAD_REQUEST,
            headers,
            Json(ConfirmRegistrationResponse::RegistrationConfirmError),
        );
    }
    let pending_registration = pending_registration.unwrap();
    let token_matches =
        crate::security::token::compare_token(&request.token, &pending_registration.confirm_token);
    if !token_matches {
        return (
            StatusCode::BAD_REQUEST,
            headers,
            Json(ConfirmRegistrationResponse::RegistrationConfirmError),
        );
    }

    // Now the registration is confirmed.
    // But check if the username/email is already taken.
    // (Normally this should not happen, but if it does, we should not allow the registration.)
    let user =
        database::entity::user::find_by_username(&app_state.db, &pending_registration.username)
            .await;
    if user.is_err() {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            headers,
            Json(ConfirmRegistrationResponse::DatabaseError),
        );
    }
    let user = user.unwrap();
    if user.is_some() {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            headers,
            Json(ConfirmRegistrationResponse::UserAlreadyExists),
        );
    }
    let user =
        database::entity::user::find_by_email(&app_state.db, &pending_registration.email).await;
    if user.is_err() {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            headers,
            Json(ConfirmRegistrationResponse::DatabaseError),
        );
    }
    let user = user.unwrap();
    if user.is_some() {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            headers,
            Json(ConfirmRegistrationResponse::UserAlreadyExists),
        );
    }

    // Now we can create the user.
    let user_creation =
        database::entity::registration::upgrade_to_user(&app_state.db, &pending_registration).await;
    if user_creation.is_err() {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            headers,
            Json(ConfirmRegistrationResponse::DatabaseError),
        );
    } else {
        let token = user_creation.unwrap();
        headers.insert(
            axum::http::header::SET_COOKIE,
            axum::http::HeaderValue::from_str(&format!("Token={token}; Path=/")).unwrap(),
        );

        return (
            StatusCode::OK,
            headers,
            Json(ConfirmRegistrationResponse::Ok { token }),
        );
    }
}
