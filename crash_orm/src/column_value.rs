//! # Types
//! The following types are valid as properties for entities.
//!
//! | Rust type             | Postgres type                                             |
//! |-----------------------|-----------------------------------------------------------|
//! | bool                  | BOOL                                                      |
//! | i8                    | CHAR                                                      |
//! | i16                   | INT2                                                      |
//! | i32                   | INT4                                                      |
//! | i64                   | INT8                                                      |
//! | u32                   | OID                                                       |
//! | f32                   | FLOAT4                                                    |
//! | f64                   | FLOAT8                                                    |
//! | String                | TEXT                                                      |
//! | rust_decimal::Decimal | NUMERIC                                                   |
//! | chrono::DateTime      | TIMESTAMP WITH TIME ZONE                                  |
//! | chrono::NaiveDateTime | TIMESTAMP                                                 |
//! | chrono::NaiveDate     | DATE                                                      |
//! | chrono::NaiveTime     | TIME                                                      |
//! | uuid::Uuid            | UUID                                                      |
//! | serde_json::Value     | JSON                                                      |
//! | crash_orm::OneToOne   | Primary key of the referenced table (Foreign Key)         |
//! | crash_orm::ManyToOne  | Primary key of the referenced table (Foreign Key)         |
//!
//! Those are not all valid types.
//! A valid type must implement ToSql and FromSql from tokio-postgres.
//!
//! To make a column nullable, just put the type in an Option.

use std::fmt::Debug;
use std::sync::Arc;

use crate::prelude::*;

/// Trait implemented on all values
///
/// This value trait is typed. For untyped values use [`UntypedColumnValue`].
pub trait TypedColumnValue<T: ColumnType>: UntypedColumnValue {}

impl<T: ColumnType, U: Entity> TypedColumnValue<T> for VirtualColumn<T, U> {}
impl<T: ColumnType, U: Entity> TypedColumnValue<T> for VirtualColumn<Option<T>, U> {}
impl<T: ColumnType, U: Entity> TypedColumnValue<T> for EntityColumn<T, U> {}
impl<T: ColumnType, U: Entity> TypedColumnValue<T> for EntityColumn<Option<T>, U> {}

impl<R: UntypedColumnValue + ColumnType> TypedColumnValue<R> for R {}

/// Trait implemented on all values
///
/// This value trait is untyped. For typed values use [`TypedColumnValue`].
pub trait UntypedColumnValue {
    /// Internal function to get a sql representation of the value
    fn get_sql(&self) -> BoxedSql;
}

macro_rules! simple_column_value {
    ($column_type:ty) => {
        impl UntypedColumnValue for $column_type {
            fn get_sql(&self) -> BoxedSql {
                BoxedSql::new("_$i".to_string(), vec![Arc::new(Box::new(self.clone()))])
            }
        }
    };
}

simple_column_value!(bool);
simple_column_value!(i8);
simple_column_value!(i16);
simple_column_value!(i32);
simple_column_value!(i64);
simple_column_value!(u32);
simple_column_value!(f32);
simple_column_value!(f64);
#[cfg(feature = "with-rust-decimal")]
simple_column_value!(rust_decimal::Decimal);
#[cfg(feature = "with-uuid")]
simple_column_value!(uuid::Uuid);
#[cfg(feature = "with-chrono")]
simple_column_value!(chrono::NaiveDateTime);
#[cfg(feature = "with-chrono")]
simple_column_value!(chrono::DateTime<chrono::Utc>);
#[cfg(feature = "with-chrono")]
simple_column_value!(chrono::DateTime<chrono::Local>);
#[cfg(feature = "with-chrono")]
simple_column_value!(chrono::DateTime<chrono::FixedOffset>);
#[cfg(feature = "with-chrono")]
simple_column_value!(chrono::NaiveDate);
#[cfg(feature = "with-chrono")]
simple_column_value!(chrono::NaiveTime);
simple_column_value!(String);
#[cfg(feature = "with-eui48")]
simple_column_value!(eui48::MacAddress);
#[cfg(feature = "with-time")]
simple_column_value!(time::PrimitiveDateTime);
#[cfg(feature = "with-time")]
simple_column_value!(time::OffsetDateTime);
#[cfg(feature = "with-time")]
simple_column_value!(time::Date);
#[cfg(feature = "with-time")]
simple_column_value!(time::Time);
#[cfg(feature = "with-geo-types")]
simple_column_value!(geo_types::Point);
#[cfg(feature = "with-geo-types")]
simple_column_value!(geo_types::Rect);
#[cfg(feature = "with-geo-types")]
simple_column_value!(geo_types::LineString);

impl<JSON: serde::Serialize + serde::de::DeserializeOwned + Debug + Clone + Send + Sync + 'static> UntypedColumnValue for TypedJson<JSON> {
    fn get_sql(&self) -> BoxedSql {
        BoxedSql::new("_$i".to_string(), vec![Arc::new(Box::new(self.clone()))])
    }
}

impl<T: ColumnType, U: Entity> UntypedColumnValue for VirtualColumn<T, U> {
    fn get_sql(&self) -> BoxedSql {
        self.get_sql()
    }
}

impl<T: ColumnType, U: Entity> UntypedColumnValue for EntityColumn<T, U> {
    fn get_sql(&self) -> BoxedSql {
        self.get_sql()
    }
}

impl<T: UntypedColumnValue> UntypedColumnValue for Option<T> {
    fn get_sql(&self) -> BoxedSql {
        if self.is_some() {
            self.as_ref().unwrap().get_sql()
        } else {
            BoxedSql::new(String::from("NULL"), vec![])
        }
    }
}