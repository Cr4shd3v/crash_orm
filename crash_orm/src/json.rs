//! This module contains a simple json wrapper type for use in the ORM.

use std::error::Error;
use std::fmt::Debug;
use std::ops::{Deref, DerefMut};
use postgres::types::{to_sql_checked, FromSql, IsNull, ToSql, Type};
use postgres::types::private::BytesMut;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_json::Value;

/// This is a basic wrapper type to provide typed json effortlessly.
#[derive(Debug, Clone)]
pub struct TypedJson<T: Serialize + DeserializeOwned + Debug + Clone + Send + Sync + 'static>(pub T);

impl<T: Serialize + DeserializeOwned + Debug + Clone + Send + Sync + 'static> Serialize for TypedJson<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer
    {
        self.0.serialize(serializer)
    }
}

impl<'a, T: Serialize + DeserializeOwned + Debug + Clone + Send + Sync + 'static> Deserialize<'a> for TypedJson<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'a>
    {
        Ok(TypedJson(T::deserialize(deserializer)?))
    }
}

impl<T: Serialize + DeserializeOwned + Debug + Clone + Send + Sync + 'static> Deref for TypedJson<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: Serialize + DeserializeOwned + Debug + Clone + Send + Sync + 'static> DerefMut for TypedJson<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T: Serialize + DeserializeOwned + Debug + Clone + Send + Sync + 'static> ToSql for TypedJson<T> {
    fn to_sql(&self, ty: &Type, out: &mut BytesMut) -> Result<IsNull, Box<dyn Error + Sync + Send>>
    where
        Self: Sized
    {
        serde_json::to_value(self)?.to_sql(ty, out)
    }

    fn accepts(ty: &Type) -> bool
    where
        Self: Sized
    {
        <Value as ToSql>::accepts(ty)
    }

    to_sql_checked!();
}

impl<'a, T: Serialize + DeserializeOwned + Debug + Clone + Send + Sync + 'static> FromSql<'a> for TypedJson<T> {
    fn from_sql(ty: &Type, raw: &'a [u8]) -> Result<Self, Box<dyn Error + Sync + Send>> {
        Ok(TypedJson(serde_json::from_value(<Value as FromSql>::from_sql(ty, raw)?)?))
    }

    fn accepts(ty: &Type) -> bool {
        <Value as FromSql>::accepts(ty)
    }
}

impl<T: Serialize + DeserializeOwned + Debug + Clone + Send + Sync + 'static> From<T> for TypedJson<T> {
    fn from(value: T) -> Self {
        Self(value)
    }
}
