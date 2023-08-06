use std::net::IpAddr;

use axum::{
    extract::{Path, State},
    http::{HeaderMap, StatusCode},
    Json,
};
use crypto::token::{compare_token, generate_token};
use sqlx::query;

use crate::{
    v1::{auth::register::registration_expiration, ResultResponse},
    AppState,
};

use api_types::{v1::confirm_register::*, Snowflake};

const TOKEN_LENGTH: u16 = 32;

pub async fn confirm_registration(
    State(app_state): State<AppState>,
    Path(reg_id): Path<Snowflake>,
    Json(request): Json<ConfirmRegistrationRequest>,
) -> ResultResponse<(StatusCode, HeaderMap, Json<ConfirmRegistrationResponse>)> {
    let db = &app_state.db;
    let mut headers = HeaderMap::new();
    let pending_registration = query!("SELECT * FROM registration WHERE id=?", reg_id)
        .fetch_optional(db)
        .await?;
    if pending_registration.is_none() {
        return Ok((
            StatusCode::BAD_REQUEST,
            headers,
            Json(ConfirmRegistrationResponse::RegistrationConfirmError),
        ));
    }
    let pending_registration = pending_registration.unwrap();
    let token_matches = compare_token(&request.token, &pending_registration.confirm_token);
    if !token_matches {
        return Ok((
            StatusCode::BAD_REQUEST,
            headers,
            Json(ConfirmRegistrationResponse::RegistrationConfirmError),
        ));
    }

    // Now the registration is confirmed.
    // But check if the username/email is already taken.
    // (Normally this should not happen, but if it does, we should not allow the registration.)
    let user = query!(
        "SELECT id FROM user WHERE username=? OR email=?",
        pending_registration.username,
        pending_registration.email,
    )
    .fetch_optional(db)
    .await?;
    if user.is_some() {
        return Ok((
            StatusCode::INTERNAL_SERVER_ERROR,
            headers,
            Json(ConfirmRegistrationResponse::UserAlreadyExists),
        ));
    }

    // Now we can create the user.
    let new_token = {
        // Transactionally delete the registration and create the user. Return the instance of the new token created for the user.
        let mut tx = db.begin().await?;
        // Delete registration
        query!("DELETE FROM registration WHERE id=?", reg_id)
            .execute(&mut tx)
            .await?;
        // Create user
        query!(
            "INSERT INTO user (id, username, email, password_hash) VALUES (?,?,?,?)",
            pending_registration.id,
            pending_registration.username,
            pending_registration.email,
            pending_registration.password_hash,
        )
        .execute(&mut tx)
        .await?;

        // Make token
        let new_token = generate_token(TOKEN_LENGTH);
        let id = Snowflake::new().await;
        let now = id.timestamp();
        let expires = (now + registration_expiration()).timestamp();
        let created_by_ip = pending_registration
            .created_by_ip
            .parse()
            .unwrap_or(IpAddr::V4(std::net::Ipv4Addr::new(0, 0, 0, 0)))
            .to_string(); // This is not expected to fail because it comes from the database
        query!("INSERT INTO user_token (id, token, user_id, created_by_ip, expires_unix_time) VALUES (?,?,?,?,?)",
            id,
            new_token,
            pending_registration.id,
            created_by_ip,
            expires
        ).execute(&mut tx).await?;
        tx.commit().await?;
        new_token
    };

    headers.insert(
        axum::http::header::SET_COOKIE,
        axum::http::HeaderValue::from_str(&format!("Token={new_token}; Path=/")).unwrap(),
    );

    return Ok((
        StatusCode::OK,
        headers,
        Json(ConfirmRegistrationResponse::Ok { token: new_token }),
    ));
}
