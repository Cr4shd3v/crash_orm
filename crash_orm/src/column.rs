use rust_decimal::Decimal;
use tokio_postgres::types::ToSql;
use crate::{Entity, EntityColumn, VirtualColumn};

/// Trait implemented on all Columns
///
/// This column trait is typed. For untyped columns use [`UntypedColumn`].
pub trait Column<T: ToSql, U: Entity<U>>: UntypedColumn<U> {}

impl<T: ToSql + Sync, U: Entity<U> + Sync> Column<T, U> for VirtualColumn<T, U> {}
impl<T: ToSql + Sync, U: Entity<U> + Sync> Column<T, U> for VirtualColumn<Option<T>, U> {}
impl<T: ToSql + Sync, U: Entity<U> + Sync> Column<T, U> for EntityColumn<T, U> {}
impl<T: ToSql + Sync, U: Entity<U> + Sync> Column<T, U> for EntityColumn<Option<T>, U> {}

/// Trait implemented on all Columns
///
/// This column trait is untyped. For typed columns use [`Column`].
pub trait UntypedColumn<U: Entity<U>>: Sync {
    /// Internal function to get a sql representation of the column
    fn get_sql(&self) -> String;
}

impl<T: ToSql + Sync, U: Entity<U> + Sync> UntypedColumn<U> for EntityColumn<T, U> {
    fn get_sql(&self) -> String {
        self.get_sql()
    }
}

impl<T: ToSql + Sync, U: Entity<U> + Sync> UntypedColumn<U> for VirtualColumn<T, U> {
    fn get_sql(&self) -> String {
        self.get_sql()
    }
}

/// Trait implemented on all values
///
/// This value trait is typed. For untyped values use [`UntypedColumnValue`].
pub trait ColumnValue<T: ToSql>: UntypedColumnValue {}

impl<T: ToSql, U: Entity<U>> ColumnValue<T> for VirtualColumn<T, U> {}
impl<T: ToSql, U: Entity<U>> ColumnValue<T> for VirtualColumn<Option<T>, U> {}
impl<T: ToSql, U: Entity<U>> ColumnValue<T> for EntityColumn<T, U> {}
impl<T: ToSql, U: Entity<U>> ColumnValue<T> for EntityColumn<Option<T>, U> {}

impl<R: UntypedColumnValue + ToSql> ColumnValue<R> for R {}

/// Trait implemented on all values
///
/// This value trait is untyped. For typed values use [`ColumnValue`].
pub trait UntypedColumnValue {
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