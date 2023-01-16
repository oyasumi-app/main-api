mod create;
mod event_stream;
mod get;
mod list;

use crate::AppState;
use axum::Router;

pub fn get_router() -> Router<AppState> {
    Router::new().nest("/stream", event_stream::get_router())
}
