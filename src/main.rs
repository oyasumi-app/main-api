mod api;
mod security;

mod v1;

pub use api::AppState;
pub use api_types::snowflake::*;
pub use security::http_auth::{ExtractUser, RequireUser, User};

//use trace::trace;

//trace::init_depth_var!();

fn main() {
    api::main();
}
