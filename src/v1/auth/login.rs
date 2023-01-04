use axum::extract::{Json, State};

use crate::core::query;
use crate::AppState;
use axum_client_ip::ClientIp;

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

#[axum_macros::debug_handler]
pub async fn login(
    State(app_state): State<AppState>,
    ClientIp(ip): ClientIp,
    Json(request): Json<LoginRequest>,
) -> Json<LoginResponse> {
    let login_id;
    let pw;
    match request {
        LoginRequest::UsernamePassword { username, password } => {
            login_id = query::LoginIdentifier::Username(username);
            pw = password;
        }
        LoginRequest::EmailPassword { email, password } => {
            login_id = query::LoginIdentifier::Email(email);
            pw = password;
        }
    };

    let user = query::Query::get_user_by_login(&app_state.db, login_id, &pw).await;

    if user.is_none() {
        return Json(LoginResponse::Err(LoginError::InvalidCredentials));
    }

    let user = user.unwrap();

    let token = crate::entity::user_token::make_token(&app_state.db, &user, &ip)
        .await
        .unwrap();

    Json(LoginResponse::Ok { token })
}
