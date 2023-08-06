use std::{
    fmt::{Display, Formatter},
    str::FromStr,
    time::Duration,
};

use std::time::UNIX_EPOCH;

#[cfg(feature = "snowflake_create")]
use async_std::sync::Mutex;

use chrono::{DateTime, Utc};

use serde::{Deserialize, Serialize};

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
#[cfg(feature = "snowflake_create")]
static SNOWFLAKE_COUNTER: Mutex<u8> = Mutex::new(0);

impl Snowflake {
    /// Create a new Snowflake.
    ///
    /// This function is thread-safe.
    #[cfg(feature = "snowflake_create")]
    pub async fn new() -> Self {
        use std::time::SystemTime;

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
pub fn u64_to_i64(x: u64) -> i64 {
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

impl Display for Snowflake {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for Snowflake {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.parse().map(Snowflake)
    }
}

// SeaORM integration starts here

#[cfg(feature = "sqlx_integration")]
use sqlx::{Decode, Encode, Sqlite, Type};

#[cfg(feature = "sqlx_integration")]
impl Type<Sqlite> for Snowflake {
    fn type_info() -> <Sqlite as sqlx::Database>::TypeInfo {
        i64::type_info()
    }
}

#[cfg(feature = "sqlx_integration")]
impl Encode<'_, Sqlite> for Snowflake {
    fn encode_by_ref(
        &self,
        buf: &mut <Sqlite as sqlx::database::HasArguments<'_>>::ArgumentBuffer,
    ) -> sqlx::encode::IsNull {
        self.0.encode_by_ref(buf)
    }
}

#[cfg(feature = "sqlx_integration")]
impl Decode<'_, Sqlite> for Snowflake {
    fn decode(
        value: <Sqlite as sqlx::database::HasValueRef<'_>>::ValueRef,
    ) -> Result<Self, sqlx::error::BoxDynError> {
        Ok(Snowflake::from(i64::decode(value)?))
    }
}
