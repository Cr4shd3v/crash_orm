use tokio_postgres::types::ToSql;

use crate::{BoxedColumnValue, Entity, EntityColumn, VirtualColumn};
use crate::primary::PrimaryKeyType;

/// Trait implemented on all column structs of an entity containing the id.
pub trait BaseColumn<U: Entity<U, P>, P: PrimaryKeyType> {
    /// [EntityColumn] for the id of the entity.
    const ID: EntityColumn<P, U, P> = EntityColumn::<P, U, P>::new("id");
}

/// Trait implemented on all Columns
///
/// This column trait is typed. For untyped columns use [`UntypedColumn`].
pub trait Column<T: ToSql, U: Entity<U, P>, P: PrimaryKeyType>: UntypedColumn<U, P> {}

impl<T: ToSql + Sync, U: Entity<U, P> + Sync, P: PrimaryKeyType> Column<T, U, P> for VirtualColumn<T, U, P> {}
impl<T: ToSql + Sync, U: Entity<U, P> + Sync, P: PrimaryKeyType> Column<T, U, P> for VirtualColumn<Option<T>, U, P> {}
impl<T: ToSql + Sync, U: Entity<U, P> + Sync, P: PrimaryKeyType> Column<T, U, P> for EntityColumn<T, U, P> {}
impl<T: ToSql + Sync, U: Entity<U, P> + Sync, P: PrimaryKeyType> Column<T, U, P> for EntityColumn<Option<T>, U, P> {}

/// Trait implemented on all Columns
///
/// This column trait is untyped. For typed columns use [`Column`].
pub trait UntypedColumn<U: Entity<U, P>, P: PrimaryKeyType>: Sync {
    /// Internal function to get a sql representation of the column
    fn get_sql(&self) -> BoxedColumnValue;
}

impl<T: ToSql + Sync, U: Entity<U, P> + Sync, P: PrimaryKeyType> UntypedColumn<U, P> for EntityColumn<T, U, P> {
    fn get_sql(&self) -> BoxedColumnValue {
        self.get_sql()
    }
}

impl<T: ToSql + Sync, U: Entity<U, P> + Sync, P: PrimaryKeyType> UntypedColumn<U, P> for VirtualColumn<T, U, P> {
    fn get_sql(&self) -> BoxedColumnValue {
        self.get_sql()
    }
}
