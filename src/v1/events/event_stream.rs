mod create;
mod get;
mod list;
mod patch;

use crate::AppState;
use axum::{
    routing::{get, post},
    Router,
};

pub fn get_router() -> Router<AppState> {
    Router::new()
        .route("/", get(list::list_event_streams))
        .route("/:id", get(get::get_event_stream).patch(patch::patch_event_stream))
        .route("/new", post(create::new_event_stream))
}
