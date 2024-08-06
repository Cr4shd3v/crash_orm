//! Contains traits defining columns as typed or untyped traits.

use tokio_postgres::types::ToSql;

use crate::prelude::{BoxedSql, Entity, EntityColumn, VirtualColumn};

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
    fn get_sql(&self) -> BoxedSql;
}

impl<T: ToSql + Sync, U: Entity<U> + Sync> UntypedColumn<U> for EntityColumn<T, U> {
    fn get_sql(&self) -> BoxedSql {
        self.get_sql()
    }
}

impl<T: ToSql + Sync, U: Entity<U> + Sync> UntypedColumn<U> for VirtualColumn<T, U> {
    fn get_sql(&self) -> BoxedSql {
        self.get_sql()
    }
}
