mod create;
mod delete;
mod get;
mod list;
mod update;

use axum::{
    routing::{get, post},
    Router,
};

use crate::AppState;

use self::{
    create::create_now,
    delete::{delete_by_id, delete_current},
    get::{get_by_id, get_current},
    list::list_states,
    update::{put_by_id, set_current_end, set_current_start},
};

pub fn get_router() -> Router<AppState> {
    Router::new()
        .route("/", get(root))
        .route("/list", get(list_states))
        .route("/:id", get(get_by_id).delete(delete_by_id).put(put_by_id))
        .route("/new", post(create_now))
        .route(
            "/@current",
            get(get_current)
                .post(set_current_end)
                .put(set_current_start)
                .delete(delete_current),
        )
}

async fn root() -> &'static str {
    concat!(
        "Sleep state API\n",
        "GET /list -- list of all sleep states you have\n",
        "GET /<id> -- get sleep state by ID\n",
        "POST /new - create a sleep state whose start time is now, or 409 if current sleep state already exists\n",
        "PUT /<id> -- change sleep state by ID (ID in body must match the entry's data)\n",
        "DELETE /<id> -- delete sleep state by ID, or 404\n",
        "GET /@current -- the sleep state that is not completed, or 404\n",
        "POST /@current -- modify the current sleep state, so that its end time is now (and it is not the current sleep state anymore)\n",
        "PUT /@current -- modify the current sleep state, so that its start time is now\n",
        "DELETE /@current -- delete the current sleep state\n",
    )
}
