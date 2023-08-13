mod login;
use login::login;
mod register;
use register::{get_registration, get_registration_info, make_registration, resend_confirm_email};
mod confirm_register;
use confirm_register::confirm_registration;
mod tokens;
//use tokens::{get_token, delete_token};
mod check;
use check::check;

use crate::AppState;
use axum::{
    routing::{get, post},
    Router,
};

pub fn get_router() -> Router<AppState> {
    Router::new()
        .route("/login", post(login))
        .route(
            "/registration",
            post(make_registration).get(get_registration_info),
        )
        .route("/registration/:id", get(get_registration))
        .route("/registration/:id/confirm", post(confirm_registration))
        .route("/registration/:id/resend", post(resend_confirm_email))
        .route("/check", get(check))
        .route(
            "/token/by_id/:id",
            get(tokens::get_token).delete(tokens::delete_token),
        )
        .route(
            "/token/@me",
            get(tokens::get_current_token).delete(tokens::delete_current_token),
        )
        .route(
            "/token/by_token/:token",
            get(tokens::get_token_by_token).delete(tokens::delete_token_by_token),
        )
        .route(
            "/token/list",
            get(tokens::get_user_tokens).delete(tokens::delete_user_tokens),
        )
}
