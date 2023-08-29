use tokio_postgres::types::ToSql;
use crate::{Entity, EntityColumn, VirtualColumn};

pub trait Column<T: ToSql, U: Entity<U>> {
    fn get_sql(&self) -> String;
}

impl<T: ToSql, U: Entity<U>> Column<T, U> for VirtualColumn<T, U> {
    fn get_sql(&self) -> String {
        self.get_sql()
    }
}
impl<T: ToSql, U: Entity<U>> Column<T, U> for VirtualColumn<Option<T>, U> {
    fn get_sql(&self) -> String {
        self.get_sql()
    }
}
impl<T: ToSql, U: Entity<U>> Column<T, U> for EntityColumn<T, U> {
    fn get_sql(&self) -> String {
        self.get_sql()
    }
}
impl<T: ToSql, U: Entity<U>> Column<T, U> for EntityColumn<Option<T>, U> {
    fn get_sql(&self) -> String {
        self.get_sql()
    }
}

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

pub trait IntoColumnValue<U: Entity<U>> {
    fn get_string(&self) -> String;
}

impl<U: Entity<U>, R: UntypedColumn<U>> IntoColumnValue<U> for R {
    fn get_string(&self) -> String {
        self.get_sql()
    }
}