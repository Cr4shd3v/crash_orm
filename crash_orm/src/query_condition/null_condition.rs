use crate::{Column, Entity, QueryCondition};
use tokio_postgres::types::ToSql;
use crate::primary::PrimaryKey;

/// Trait implementing null checks
pub trait NullQueryColumn<T: ToSql, U: Entity<U, PRIMARY>, PRIMARY: PrimaryKey> {
    fn is_null(&self) -> QueryCondition<U, PRIMARY>;

    fn is_not_null(&self) -> QueryCondition<U, PRIMARY>;
}

impl<T: ToSql, U: Entity<U, PRIMARY>, C: Column<Option<T>, U, PRIMARY>, PRIMARY: PrimaryKey> NullQueryColumn<T, U, PRIMARY> for C {
    fn is_null(&self) -> QueryCondition<U, PRIMARY> {
        QueryCondition::IsNull(self.get_sql())
    }

    fn is_not_null(&self) -> QueryCondition<U, PRIMARY> {
        QueryCondition::IsNotNull(self.get_sql())
    }
}
