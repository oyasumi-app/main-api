use axum::extract::{Json, State};

use crate::AppState;
use axum_client_ip::ClientIp;
use database::core::query;

use api_types::v1::login::*;

#[axum_macros::debug_handler]
pub async fn login(
    State(app_state): State<AppState>,
    ClientIp(ip): ClientIp,
    Json(request): Json<LoginRequest>,
) -> (axum::http::HeaderMap, Json<LoginResponse>) {
    let login_id;
    let pw;
    let mut headers = axum::http::HeaderMap::new();
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
        return (
            headers,
            Json(LoginResponse::Err(LoginError::InvalidCredentials)),
        );
    }

    let user = user.unwrap();

    let token = database::entity::user_token::make_token(&app_state.db, &user, &ip)
        .await
        .unwrap();

    headers.insert(
        axum::http::header::SET_COOKIE,
        axum::http::HeaderValue::from_str(&format!("Token={token}")).unwrap(),
    );

    (headers, Json(LoginResponse::Ok { token }))
}
