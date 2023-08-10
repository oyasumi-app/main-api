use std::{
    fmt::{Debug, Formatter},
    ops::{Deref, DerefMut},
};

use serde::{Deserialize, Serialize};

/// Struct for anonymizing the value of a field in debug output.
///
/// `Anonymized<T>` behaves like `T` in all ways except for `Debug` output.
/// Its `Debug` output is a string of the form `Anonymized::<T>(...)`.
/// This is useful for showing structs with sensitive info (like passwords)
/// in debug output.
#[derive(Clone)]
pub struct Anonymized<T>(pub T);

impl<T> From<T> for Anonymized<T> {
    fn from(t: T) -> Self {
        Self(t)
    }
}

impl<T> Deref for Anonymized<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for Anonymized<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T> Debug for Anonymized<T> {
    fn fmt(&self, fmt: &mut Formatter) -> std::fmt::Result {
        fmt.debug_tuple(&format!("Anonymized::<{}>", std::any::type_name::<T>()))
            .field(&format_args!("..."))
            .finish()
    }
}

impl<T> Serialize for Anonymized<T>
where
    T: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.0.serialize(serializer)
    }
}

impl<'a, T> Deserialize<'a> for Anonymized<T>
where
    T: Deserialize<'a>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'a>,
    {
        Ok(Self(T::deserialize(deserializer)?))
    }
}

impl<T> PartialEq for Anonymized<T>
where
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.0.eq(&other.0)
    }
}
