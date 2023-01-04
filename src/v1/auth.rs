mod login;
use login::login;
mod register;
use register::register;
mod confirm_register;
use confirm_register::confirm_register;

use crate::AppState;
use axum::{routing::post, Router};

pub fn get_router() -> Router<AppState> {
    Router::new()
        .route("/login", post(login))
        .route("/register", post(register))
        .route("/confirm_register", post(confirm_register))
}
