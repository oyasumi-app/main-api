use std::{
    fmt::{Display, Formatter},
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use async_std::sync::Mutex;
use chrono::{DateTime, Utc};
use sea_orm::DbErr;
use serde::{Deserialize, Serialize};

/// Functions to deal with identifiers

/// The main identifier type for our application shall be the Snowflake:
/// a 64-bit integer that is unique across all entities,
/// that encodes the time of creation.
///
/// The first 48 bits are the timestamp in milliseconds since the Unix epoch.
/// This gives a range of about 8920 years.
///
/// The next 12 bits are reserved, and are currently set to 0.
/// The final 4 bits are the sequence number, which is incremented
/// for each new Snowflake created, and wraps around to 0, which ensures uniqueness.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Default,
)]
pub struct Snowflake(i64);

/// A mutex that holds the sequence number counter.
static SNOWFLAKE_COUNTER: Mutex<u8> = Mutex::new(0);

impl Snowflake {
    /// Create a new Snowflake.
    ///
    /// This function is thread-safe.
    pub async fn new() -> Self {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time is before 1970")
            .as_millis() as i64;
        let mut counter = SNOWFLAKE_COUNTER.lock().await;
        let snowflake = now << 16 | (*counter as i64);
        *counter = (*counter + 1) % 16;
        Snowflake(snowflake)
    }

    /// Get the timestamp from a Snowflake.
    pub fn timestamp(&self) -> DateTime<Utc> {
        let timestamp = self.0 >> 16;
        let timestamp = UNIX_EPOCH + Duration::from_millis(timestamp.try_into().unwrap());
        DateTime::<Utc>::from(timestamp)
    }
}

/// Losslessly convert an u64 to an i64, copying every bit.
/// The i64 might be negative as a result of this.
/// Has the same effect as reinterpreting the bits as an i64.
fn u64_to_i64(x: u64) -> i64 {
    let bytes = x.to_be_bytes();
    i64::from_be_bytes(bytes)
}

impl From<Snowflake> for i64 {
    fn from(snowflake: Snowflake) -> Self {
        snowflake.0
    }
}

impl From<i64> for Snowflake {
    fn from(id: i64) -> Self {
        Snowflake(id)
    }
}

impl From<Snowflake> for sea_orm::Value {
    fn from(snowflake: Snowflake) -> Self {
        sea_orm::Value::BigInt(Some(snowflake.0))
    }
}

impl Display for Snowflake {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

// SeaORM integration starts here

impl sea_orm::sea_query::ValueType for Snowflake {
    fn try_from(v: sea_orm::Value) -> Result<Self, sea_orm::sea_query::ValueTypeErr> {
        match v {
            sea_orm::Value::BigInt(Some(i)) => Ok(Snowflake(i)),
            _ => Err(sea_orm::sea_query::ValueTypeErr),
        }
    }

    fn type_name() -> String {
        "bigint".to_string()
    }

    fn array_type() -> sea_orm::sea_query::ArrayType {
        sea_orm::sea_query::ArrayType::BigInt
    }

    fn column_type() -> sea_orm::sea_query::ColumnType {
        sea_orm::sea_query::ColumnType::BigInteger(None)
    }
}

impl sea_orm::TryGetable for Snowflake {
    fn try_get(
        res: &sea_orm::QueryResult,
        pre: &str,
        col: &str,
    ) -> Result<Self, sea_orm::TryGetError> {
        i64::try_get(res, pre, col).map(Snowflake)
    }
}

impl sea_orm::TryFromU64 for Snowflake {
    fn try_from_u64(v: u64) -> Result<Self, DbErr> {
        Ok(Snowflake(u64_to_i64(v)))
    }
}
