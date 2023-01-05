mod login;
use login::login;
mod register;
use register::register;
mod confirm_register;
use confirm_register::confirm_register;
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
        .route("/register", post(register))
        .route("/confirm_register", post(confirm_register))
        .route("/check", get(check))
        .route(
            "/token/:id",
            get(tokens::get_token).delete(tokens::delete_token),
        )
        .route("/token/by_token/:token", get(tokens::get_token_by_token).delete(tokens::delete_token_by_token))
}