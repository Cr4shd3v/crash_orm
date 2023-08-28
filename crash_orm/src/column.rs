use tokio_postgres::types::ToSql;
use crate::{Entity, EntityColumn, VirtualColumn};

pub trait Column<T: ToSql, U: Entity<U> + Send + 'static> {
    fn get_name(&self) -> String;
}

impl<T: ToSql, U: Entity<U> + Send + 'static> Column<T, U> for VirtualColumn<T, U> {
    fn get_name(&self) -> String {
        self.get_name()
    }
}
impl<T: ToSql, U: Entity<U> + Send + 'static> Column<T, U> for VirtualColumn<Option<T>, U> {
    fn get_name(&self) -> String {
        self.get_name()
    }
}
impl<T: ToSql, U: Entity<U> + Send + 'static> Column<T, U> for EntityColumn<T, U> {
    fn get_name(&self) -> String {
        self.get_name()
    }
}
impl<T: ToSql, U: Entity<U> + Send + 'static> Column<T, U> for EntityColumn<Option<T>, U> {
    fn get_name(&self) -> String {
        self.get_name()
    }
}

pub trait UntypedColumn<U: Entity<U> + Send + 'static> {
    fn get_sql(&self) -> String;
}

impl<T: ToSql, U: Entity<U> + Send + 'static> UntypedColumn<U> for EntityColumn<T, U> {
    fn get_sql(&self) -> String {
        self.get_name()
    }
}
impl<T: ToSql, U: Entity<U> + Send + 'static> UntypedColumn<U> for VirtualColumn<T, U> {
    fn get_sql(&self) -> String {
        self.get_name()
    }
}