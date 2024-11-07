//! Contains traits defining columns as typed or untyped traits.

use crate::prelude::{BoxedSql, ColumnType, Entity, EntityColumn, VirtualColumn};

/// Trait implemented on all Columns
///
/// This column trait is typed. For untyped columns use [`UntypedColumn`].
pub trait Column<T: ColumnType, U: Entity>: UntypedColumn<U> {}

impl<T: ColumnType, U: Entity + Sync> Column<T, U> for VirtualColumn<T, U> {}
impl<T: ColumnType, U: Entity + Sync> Column<T, U> for VirtualColumn<Option<T>, U> {}
impl<T: ColumnType, U: Entity + Sync> Column<T, U> for EntityColumn<T, U> {}
impl<T: ColumnType, U: Entity + Sync> Column<T, U> for EntityColumn<Option<T>, U> {}

/// Trait implemented on all Columns
///
/// This column trait is untyped. For typed columns use [`Column`].
pub trait UntypedColumn<U: Entity>: Sync {
    /// Internal function to get a sql representation of the column
    fn get_sql(&self) -> BoxedSql;
}

impl<T: ColumnType, U: Entity + Sync> UntypedColumn<U> for EntityColumn<T, U> {
    fn get_sql(&self) -> BoxedSql {
        self.get_sql()
    }
}

impl<T: ColumnType, U: Entity + Sync> UntypedColumn<U> for VirtualColumn<T, U> {
    fn get_sql(&self) -> BoxedSql {
        self.get_sql()
    }
}
