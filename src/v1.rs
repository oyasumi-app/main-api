
use axum::{Router, extract::State, routing::get};

use crate::api::AppState;

pub fn get_router() -> Router<AppState> {
    Router::new()
        .route("/", get(root))
}

async fn root(state: State<AppState>) -> &'static str {
    "V1"
}