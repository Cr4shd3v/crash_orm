use std::rc::Rc;
use rust_decimal::Decimal;
use tokio_postgres::types::ToSql;
use crate::{Entity, EntityColumn, VirtualColumn};

#[derive(Clone)]
pub struct BoxedColumnValue {
    pub value: Rc<Box<dyn ToSql + Sync + Send>>,
}

impl BoxedColumnValue {
    pub fn new(value: Box<dyn ToSql + Sync + Send>) -> Self {
        Self {
            value: Rc::new(value),
        }
    }
}

/// Trait implemented on all values
///
/// This value trait is typed. For untyped values use [`UntypedColumnValue`].
pub trait TypedColumnValue<T: ToSql>: UntypedColumnValue {}

impl<T: ToSql, U: Entity<U>> TypedColumnValue<T> for VirtualColumn<T, U> {}
impl<T: ToSql, U: Entity<U>> TypedColumnValue<T> for VirtualColumn<Option<T>, U> {}
impl<T: ToSql, U: Entity<U>> TypedColumnValue<T> for EntityColumn<T, U> {}
impl<T: ToSql, U: Entity<U>> TypedColumnValue<T> for EntityColumn<Option<T>, U> {}

impl<R: UntypedColumnValue + ToSql> TypedColumnValue<R> for R {}

/// Trait implemented on all values
///
/// This value trait is untyped. For typed values use [`TypedColumnValue`].
pub trait UntypedColumnValue {
    /// Internal function to get a sql representation of the value
    fn get_sql(&self) -> String;
}

impl UntypedColumnValue for String {
    fn get_sql(&self) -> String {
        format!("'{}'", self)
    }
}

macro_rules! simple_column_value {
    ($column_type:ty) => {
        impl UntypedColumnValue for $column_type {
            fn get_sql(&self) -> String {
                self.to_string()
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
simple_column_value!(Decimal);

impl<T: ToSql, U: Entity<U>> UntypedColumnValue for VirtualColumn<T, U> {
    fn get_sql(&self) -> String {
        self.get_sql()
    }
}

impl<T: ToSql, U: Entity<U>> UntypedColumnValue for EntityColumn<T, U> {
    fn get_sql(&self) -> String {
        self.get_sql()
    }
}