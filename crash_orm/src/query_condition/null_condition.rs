use tokio_postgres::types::ToSql;

use crate::{Column, Entity, QueryCondition};
use crate::primary::PrimaryKeyType;

/// Trait implementing null check [QueryCondition].
pub trait NullQueryColumn<T: ToSql, U: Entity<U, P>, P: PrimaryKeyType> {
    /// Creates [QueryCondition::IsNull] for self
    fn is_null(&self) -> QueryCondition<U, P>;

    /// Creates [QueryCondition::IsNotNull] for self
    fn is_not_null(&self) -> QueryCondition<U, P>;
}

impl<T: ToSql, U: Entity<U, P>, C: Column<Option<T>, U, P>, P: PrimaryKeyType> NullQueryColumn<T, U, P> for C {
    fn is_null(&self) -> QueryCondition<U, P> {
        QueryCondition::IsNull(self.get_sql())
    }

    fn is_not_null(&self) -> QueryCondition<U, P> {
        QueryCondition::IsNotNull(self.get_sql())
    }
}
