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

use std::sync::Arc;

use tokio_postgres::types::ToSql;

use crate::prelude::{Entity, EntityColumn, PrimaryKey, VirtualColumn};

/// Struct containing a part of a query with raw sql and values prepared for tokio-postgres.
#[derive(Clone)]
pub struct BoxedColumnValue {
    pub(crate) sql: String,
    pub(crate) value: Vec<Arc<Box<dyn ToSql + Sync + Send + 'static>>>,
}

impl BoxedColumnValue {
    /// Creates a new instance
    pub(crate) fn new(
        sql: String,
        value: Vec<Arc<Box<dyn ToSql + Sync + Send + 'static>>>,
    ) -> Self {
        Self { sql, value }
    }

    /// Resolves this value into it's parts with inserted IDs
    pub(crate) fn resolve(
        &self,
        mut index: usize,
    ) -> (String, Vec<Arc<Box<dyn ToSql + Sync + Send>>>, usize) {
        let mut sql = self.sql.clone();
        while sql.contains("_$i") {
            sql = sql.replacen("_$i", &*format!("${}", index), 1);
            index += 1;
        }

        (sql, self.value.clone(), index)
    }
}

/// Trait implemented on all values
///
/// This value trait is typed. For untyped values use [`UntypedColumnValue`].
pub trait TypedColumnValue<T: ToSql>: UntypedColumnValue {}

impl<T: ToSql, U: Entity<U, P>, P: PrimaryKey> TypedColumnValue<T> for VirtualColumn<T, U, P> {}
impl<T: ToSql, U: Entity<U, P>, P: PrimaryKey> TypedColumnValue<T> for VirtualColumn<Option<T>, U, P> {}
impl<T: ToSql, U: Entity<U, P>, P: PrimaryKey> TypedColumnValue<T> for EntityColumn<T, U, P> {}
impl<T: ToSql, U: Entity<U, P>, P: PrimaryKey> TypedColumnValue<T> for EntityColumn<Option<T>, U, P> {}

impl<R: UntypedColumnValue + ToSql> TypedColumnValue<R> for R {}

/// Trait implemented on all values
///
/// This value trait is untyped. For typed values use [`TypedColumnValue`].
pub trait UntypedColumnValue {
    /// Internal function to get a sql representation of the value
    fn get_sql(&self) -> BoxedColumnValue;
}

macro_rules! simple_column_value {
    ($column_type:ty) => {
        impl UntypedColumnValue for $column_type {
            fn get_sql(&self) -> BoxedColumnValue {
                BoxedColumnValue::new("_$i".to_string(), vec![Arc::new(Box::new(self.clone()))])
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
#[cfg(feature = "with-bit-vec")]
simple_column_value!(bit_vec::BitVec);
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

impl<T: ToSql, U: Entity<U, P>, P: PrimaryKey> UntypedColumnValue for VirtualColumn<T, U, P> {
    fn get_sql(&self) -> BoxedColumnValue {
        self.get_sql()
    }
}

impl<T: ToSql, U: Entity<U, P>, P: PrimaryKey> UntypedColumnValue for EntityColumn<T, U, P> {
    fn get_sql(&self) -> BoxedColumnValue {
        self.get_sql()
    }
}

impl<T: UntypedColumnValue> UntypedColumnValue for Option<T> {
    fn get_sql(&self) -> BoxedColumnValue {
        if self.is_some() {
            self.as_ref().unwrap().get_sql()
        } else {
            BoxedColumnValue::new(String::from("NULL"), vec![])
        }
    }
}

/// Trait for converting any type that implements [ToSql] and [UntypedColumnValue] into a [TypedColumnValue].
#[allow(clippy::wrong_self_convention)]
pub trait IntoSql<T> {
    /// Convert self into a [TypedColumnValue]
    fn into_typed_value(&self) -> &(dyn TypedColumnValue<T>);
}

impl<T: ToSql + UntypedColumnValue> IntoSql<T> for T {
    fn into_typed_value(&self) -> &(dyn TypedColumnValue<T>) {
        self
    }
}

impl<T: ToSql + UntypedColumnValue> IntoSql<T> for &T {
    fn into_typed_value(&self) -> &(dyn TypedColumnValue<T>) {
        *self
    }
}