use migration::Migrator;
use sea_orm::{Database, DatabaseConnection};
use sea_orm_migration::MigratorTrait;

pub mod core;
pub mod entity;
pub mod migration;
pub mod security;

pub use api_types::Snowflake;

pub async fn connect(db_url: &str) -> DatabaseConnection {
    let conn = Database::connect(db_url)
        .await
        .expect("Database connection failed");
    Migrator::up(&conn, None).await.unwrap();
    conn
}
