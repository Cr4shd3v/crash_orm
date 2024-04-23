use tokio_postgres::types::ToSql;

use crate::{Column, Entity, QueryCondition};
use crate::primary::PrimaryKey;

/// Trait implementing null checks
pub trait NullQueryColumn<T: ToSql, U: Entity<U, P>, P: PrimaryKey> {
    fn is_null(&self) -> QueryCondition<U, P>;

    fn is_not_null(&self) -> QueryCondition<U, P>;
}

impl<T: ToSql, U: Entity<U, P>, C: Column<Option<T>, U, P>, P: PrimaryKey> NullQueryColumn<T, U, P> for C {
    fn is_null(&self) -> QueryCondition<U, P> {
        QueryCondition::IsNull(self.get_sql())
    }

    fn is_not_null(&self) -> QueryCondition<U, P> {
        QueryCondition::IsNotNull(self.get_sql())
    }
}
