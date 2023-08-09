use axum::{
    extract::{Json, State},
    http::StatusCode,
};
use chrono::Duration;
use crypto::{password::check_hash, token::generate_token};
use sqlx::query;

use crate::{v1::ResultResponse, AppState};
use axum_client_ip::ClientIp;

use api_types::{v1::login::*, Snowflake};

pub async fn login(
    State(app_state): State<AppState>,
    ClientIp(ip): ClientIp,
    Json(request): Json<LoginRequest>,
) -> ResultResponse<
    Result<(axum::http::HeaderMap, Json<LoginSuccess>), (StatusCode, Json<LoginError>)>,
> {
    let mut headers = axum::http::HeaderMap::new();
    let user_row = match request {
        LoginRequest::EmailPassword { email, password } => {
            let user_row: Option<_> = query!("SELECT * FROM user WHERE email=?", email)
                .fetch_optional(&app_state.db)
                .await?;
            user_row.and_then(|row| check_hash(&password, &row.password_hash).then_some(row))
        }
    };

    if user_row.is_none() {
        return Ok(Err((
            StatusCode::UNAUTHORIZED,
            Json(LoginError::InvalidCredentials),
        )));
    }

    let user = user_row.unwrap();

    const TOKEN_LENGTH: u16 = 32;
    let new_token = generate_token(TOKEN_LENGTH);
    let id = Snowflake::new().await;
    let now = id.timestamp();
    let expires = (now + login_expiration()).timestamp();
    let ip_str = ip.to_string();
    query!("INSERT INTO user_token (id, token, user_id, created_by_ip, expires_unix_time) VALUES (?,?,?,?,?)",
        id,
        new_token,
        user.id,
        ip_str,
        expires
    ).execute(&app_state.db).await?;

    let token = new_token;
    headers.insert(
        axum::http::header::SET_COOKIE,
        axum::http::HeaderValue::from_str(&format!("Token={token}; Path=/")).unwrap(),
    );

    Ok(Ok((headers, Json(LoginSuccess { token }))))
}

fn login_expiration() -> Duration {
    Duration::days(14)
}
