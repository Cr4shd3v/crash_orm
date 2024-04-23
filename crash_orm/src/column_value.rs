use std::sync::Arc;

use tokio_postgres::types::ToSql;

use crate::{Entity, EntityColumn, PrimaryKey, VirtualColumn};

#[derive(Clone)]
pub struct BoxedColumnValue {
    pub sql: String,
    pub value: Vec<Arc<Box<dyn ToSql + Sync + Send + 'static>>>,
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

impl<T: ToSql, U: Entity<U, PRIMARY>, PRIMARY: PrimaryKey> TypedColumnValue<T> for VirtualColumn<T, U, PRIMARY> {}
impl<T: ToSql, U: Entity<U, PRIMARY>, PRIMARY: PrimaryKey> TypedColumnValue<T> for VirtualColumn<Option<T>, U, PRIMARY> {}
impl<T: ToSql, U: Entity<U, PRIMARY>, PRIMARY: PrimaryKey> TypedColumnValue<T> for EntityColumn<T, U, PRIMARY> {}
impl<T: ToSql, U: Entity<U, PRIMARY>, PRIMARY: PrimaryKey> TypedColumnValue<T> for EntityColumn<Option<T>, U, PRIMARY> {}

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

impl<T: ToSql, U: Entity<U, PRIMARY>, PRIMARY: PrimaryKey> UntypedColumnValue for VirtualColumn<T, U, PRIMARY> {
    fn get_sql(&self) -> BoxedColumnValue {
        self.get_sql()
    }
}

impl<T: ToSql, U: Entity<U, PRIMARY>, PRIMARY: PrimaryKey> UntypedColumnValue for EntityColumn<T, U, PRIMARY> {
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

#[allow(clippy::wrong_self_convention)]
pub trait IntoSql<T> {
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