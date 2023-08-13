use serde::{Deserialize, Serialize};

use crate::Snowflake;

use super::DateTimeUtc;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct SleepState {
    pub id: Snowflake,
    pub start: DateTimeUtc,

    /// If this is empty, then the sleep state is not over yet.
    pub end: Option<DateTimeUtc>,

    pub comment: Option<String>,
}
