use database::core::sea_orm::DatabaseConnection;

use axum::middleware::from_fn_with_state;
use axum::{routing::get, Router};

use std::net::SocketAddr;

#[derive(Clone)]
pub struct AppState {
    pub db: DatabaseConnection,
}

#[tokio::main]
pub async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    dotenvy::dotenv().ok();
    let db_url = std::env!("DATABASE_URL");

    let conn = database::connect(&db_url).await;

    let app_state = AppState { db: conn };

    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(root))
        .nest("/v1", crate::v1::get_router())
        .with_state(app_state.clone())
        .route_layer(from_fn_with_state(
            app_state,
            crate::security::http_auth::auth,
        ))
        // Allow CORS with any origin and credentials
        .layer(tower_http::cors::CorsLayer::very_permissive());
    //.layer(middleware::from_fn_async(crate::security::http_auth::auth));

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service_with_connect_info::<SocketAddr>())
        .await
        .unwrap();
}

// basic handler that responds with a static string
async fn root() -> &'static str {
    "Hello, World!"
}
