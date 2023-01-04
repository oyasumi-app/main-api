use axum::{
    routing::{get, post},
    http::StatusCode,
    response::IntoResponse,
    Json, Router, extract::State,
};
use crate::migration::{Migrator, MigratorTrait};
use serde::{Deserialize, Serialize};
use crate::core::{sea_orm::{Database, DatabaseConnection}, Mutation};
use std::net::SocketAddr;
use std::env;

use sea_orm::EntityTrait;
use sea_orm::ColumnTrait;
use sea_orm::QueryFilter;

#[derive(Clone)]
struct AppState {
    db: DatabaseConnection,
    apikey: String,
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

    // Print all users, and their tokens
    let users = crate::entity::user::Entity::find()
        .all(&conn)
        .await
        .unwrap();
    for user in users {
        println!("User: {}", user.username);
        let tokens = crate::entity::user_token::Entity::find()
            .filter(crate::entity::user_token::Column::User.eq(user.id))
            .all(&conn)
            .await
            .unwrap();
        for token in tokens {
            println!("  Token: {}", token.token);
        }
    }


    let app_state = AppState {
        db: conn,
        apikey: "".to_string(),
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
    apikey: String,
}

// record a state change
async fn record_state_change(
    state: State<AppState>,
    Json(StateChange{new_state, apikey}): Json<StateChange>,
) -> impl IntoResponse {
    let db = &state.db;
    if apikey != state.apikey {
        // Show an error
        return (StatusCode::UNAUTHORIZED, "Invalid API key");
    }
    Mutation::insert_state_change(db, new_state).await.unwrap();
    (StatusCode::OK, "State change recorded")
}