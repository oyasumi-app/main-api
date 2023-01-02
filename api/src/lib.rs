use axum::{
    routing::{get, post},
    http::StatusCode,
    response::IntoResponse,
    Json, Router, extract::State,
};
use migration::{Migrator, MigratorTrait};
use serde::{Deserialize, Serialize};
use core::{sea_orm::{Database, DatabaseConnection}, Mutation};
use std::net::SocketAddr;
use std::env;

#[derive(Clone)]
struct AppState {
    db: DatabaseConnection,
}

#[tokio::main]
pub async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    dotenvy::dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");

    let conn = Database::connect(db_url)
        .await
        .expect("Database connection failed");
    Migrator::up(&conn, None).await.unwrap();

    let app_state = AppState {
        db: conn,
    };

    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(root))
        .route("/record", post(record_state_change))
        .with_state(app_state);

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// basic handler that responds with a static string
async fn root() -> &'static str {
    "Hello, World!"
}

#[derive(Deserialize)]
struct StateChange {
    new_state: bool,
}

// record a state change
async fn record_state_change(
    state: State<AppState>,
    Json(StateChange{new_state}): Json<StateChange>,
) -> impl IntoResponse {
    let db = &state.db;
    Mutation::insert_state_change(db, new_state).await.unwrap();
    StatusCode::OK
}