use crate::Snowflake;

/// Trait for objects that are identified by a Snowflake.
pub trait Snowflakable {
    fn get_id(&self) -> Snowflake;
}

/// This macro automatically adds a Snowflakable implementation from the `id` field of the struct.
#[macro_export]
macro_rules! trivially_snowflakable (
    ($name: ident) => {
        impl ::api_types::common::snowflakable::Snowflakable for $name {
            fn get_id(&self) -> ::api_types::common::snowflake::Snowflake {
                self.id
            }
        }
    }
);
