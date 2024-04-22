use tokio_postgres::types::ToSql;

use crate::{BoxedColumnValue, Entity, EntityColumn, VirtualColumn};
use crate::primary::PrimaryKey;

pub trait BaseColumn<U: Entity<U, PRIMARY>, PRIMARY: PrimaryKey<'static>> {
    const ID: EntityColumn<PRIMARY, U, PRIMARY> = EntityColumn::<PRIMARY, U, PRIMARY>::new("id");
}

/// Trait implemented on all Columns
///
/// This column trait is typed. For untyped columns use [`UntypedColumn`].
pub trait Column<T: ToSql, U: Entity<U, PRIMARY>, PRIMARY: PrimaryKey<'static>>: UntypedColumn<U, PRIMARY> {}

impl<T: ToSql + Sync, U: Entity<U, PRIMARY> + Sync, PRIMARY: PrimaryKey<'static>> Column<T, U, PRIMARY> for VirtualColumn<T, U, PRIMARY> {}
impl<T: ToSql + Sync, U: Entity<U, PRIMARY> + Sync, PRIMARY: PrimaryKey<'static>> Column<T, U, PRIMARY> for VirtualColumn<Option<T>, U, PRIMARY> {}
impl<T: ToSql + Sync, U: Entity<U, PRIMARY> + Sync, PRIMARY: PrimaryKey<'static>> Column<T, U, PRIMARY> for EntityColumn<T, U, PRIMARY> {}
impl<T: ToSql + Sync, U: Entity<U, PRIMARY> + Sync, PRIMARY: PrimaryKey<'static>> Column<T, U, PRIMARY> for EntityColumn<Option<T>, U, PRIMARY> {}

/// Trait implemented on all Columns
///
/// This column trait is untyped. For typed columns use [`Column`].
pub trait UntypedColumn<U: Entity<U, PRIMARY>, PRIMARY: PrimaryKey<'static>>: Sync {
    /// Internal function to get a sql representation of the column
    fn get_sql(&self) -> BoxedColumnValue;
}

impl<T: ToSql + Sync, U: Entity<U, PRIMARY> + Sync, PRIMARY: PrimaryKey<'static>> UntypedColumn<U, PRIMARY> for EntityColumn<T, U, PRIMARY> {
    fn get_sql(&self) -> BoxedColumnValue {
        self.get_sql()
    }
}

impl<T: ToSql + Sync, U: Entity<U, PRIMARY> + Sync, PRIMARY: PrimaryKey<'static>> UntypedColumn<U, PRIMARY> for VirtualColumn<T, U, PRIMARY> {
    fn get_sql(&self) -> BoxedColumnValue {
        self.get_sql()
    }
}
