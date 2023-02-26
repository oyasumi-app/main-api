mod auth;
mod error;
pub use error::*;

use axum::{routing::get, Router};

use crate::AppState;

pub fn get_router() -> Router<AppState> {
    Router::new()
        .route("/", get(root))
        .nest("/auth", crate::v1::auth::get_router())
}

async fn root() -> &'static str {
    "V1"
}
