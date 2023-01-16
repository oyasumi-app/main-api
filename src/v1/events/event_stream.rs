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
        .route("/list", get(list::list_event_streams))
        .route("/:stream_id/event", post(super::create::create_event))
        .route("/:stream_id/event/list", get(super::list::list_events))
        .route("/:stream_id/event/:event_id", get(super::get::get_event))
        .route(
            "/:stream_id",
            get(get::get_event_stream).patch(patch::patch_event_stream),
        )
        .route("/new", post(create::new_event_stream))
}
