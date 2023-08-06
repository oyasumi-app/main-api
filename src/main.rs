mod api;
mod security;

mod v1;

pub use api::AppState;
pub use api_types::snowflake::*;
use chrono::{DateTime, NaiveDateTime, Utc};
pub use security::http_auth::{ExtractUser, LoginState, RequireUser};

pub type DateTimeUtc = DateTime<Utc>;

//use trace::trace;

//trace::init_depth_var!();
pub fn datetime_utc_from_timestamp(timestamp: i64) -> DateTimeUtc {
    DateTimeUtc::from_utc(
        NaiveDateTime::from_timestamp_opt(timestamp, 0)
            .expect("Encountered invalid Unix timestamp"),
        Utc,
    )
}
fn main() {
    api::main();
}
