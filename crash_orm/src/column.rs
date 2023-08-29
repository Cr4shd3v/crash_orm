use tokio_postgres::types::private::BytesMut;
use tokio_postgres::types::{ToSql, Type};
use crate::{Entity, EntityColumn, VirtualColumn};

pub trait Column<T: ToSql, U: Entity<U>>: UntypedColumn<U> {
}

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

impl<T: ToSql> UntypedColumnValue for T {
    fn get_sql(&self) -> String {
        let mut bytes = BytesMut::new();
        self.to_sql_checked(&Type::ANY, &mut bytes).unwrap();
        String::from_utf8(bytes.to_vec()).unwrap()
    }
}

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