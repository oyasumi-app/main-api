mod api;
mod core;
mod entity;
mod migration;
mod security;

mod v1;

pub use api::AppState;
pub use api_types::snowflake::*;
pub use security::http_auth::{ExtractUser, RequireUser, User};

fn main() {
    api::main();
}
