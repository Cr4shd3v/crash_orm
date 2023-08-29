use rust_decimal::Decimal;
use tokio_postgres::types::ToSql;
use crate::{Entity, EntityColumn, VirtualColumn};

pub trait Column<T: ToSql, U: Entity<U>>: UntypedColumn<U> {}

impl<T: ToSql + Sync, U: Entity<U> + Sync> Column<T, U> for VirtualColumn<T, U> {}
impl<T: ToSql + Sync, U: Entity<U> + Sync> Column<T, U> for VirtualColumn<Option<T>, U> {}
impl<T: ToSql + Sync, U: Entity<U> + Sync> Column<T, U> for EntityColumn<T, U> {}
impl<T: ToSql + Sync, U: Entity<U> + Sync> Column<T, U> for EntityColumn<Option<T>, U> {}

pub trait UntypedColumn<U: Entity<U>>: Sync {
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

pub trait IntoColumnValue<T: ToSql>: UntypedColumnValue {}

impl<T: ToSql, U: Entity<U>> IntoColumnValue<T> for VirtualColumn<T, U> {}
impl<T: ToSql, U: Entity<U>> IntoColumnValue<T> for VirtualColumn<Option<T>, U> {}
impl<T: ToSql, U: Entity<U>> IntoColumnValue<T> for EntityColumn<T, U> {}
impl<T: ToSql, U: Entity<U>> IntoColumnValue<T> for EntityColumn<Option<T>, U> {}

impl<R: UntypedColumnValue + ToSql> IntoColumnValue<R> for R {}

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