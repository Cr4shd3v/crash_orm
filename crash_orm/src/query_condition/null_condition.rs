use tokio_postgres::types::ToSql;
use crate::{Column, Entity, QueryCondition};

/// Trait implementing null checks
pub trait NullQueryColumn<T: ToSql, U: Entity<U>> {
    fn is_null(&self) -> QueryCondition<U>;

    fn is_not_null(&self) -> QueryCondition<U>;
}

impl<T: ToSql, U: Entity<U>, C: Column<Option<T>, U>> NullQueryColumn<T, U> for C {
    fn is_null(&self) -> QueryCondition<U> {
        QueryCondition::IsNull(self.get_sql())
    }

    fn is_not_null(&self) -> QueryCondition<U> {
        QueryCondition::IsNotNull(self.get_sql())
    }
}