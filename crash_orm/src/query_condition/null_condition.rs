use tokio_postgres::types::ToSql;

use crate::prelude::{Column, Entity, PrimaryKey, QueryCondition};

/// Trait implementing null check [QueryCondition].
pub trait NullQueryColumn<T: ToSql, U: Entity<U, P>, P: PrimaryKey> {
    /// Creates [QueryCondition::IsNull] for self
    fn is_null(&self) -> QueryCondition<U, P>;

    /// Creates [QueryCondition::IsNotNull] for self
    fn is_not_null(&self) -> QueryCondition<U, P>;
}

impl<T: ToSql, U: Entity<U, P>, C: Column<Option<T>, U, P>, P: PrimaryKey> NullQueryColumn<T, U, P> for C {
    fn is_null(&self) -> QueryCondition<U, P> {
        let mut boxed = self.get_sql();
        boxed.modify(|v| format!("{v} IS NULL"));

        QueryCondition::new(boxed)
    }

    fn is_not_null(&self) -> QueryCondition<U, P> {
        let mut boxed = self.get_sql();
        boxed.modify(|v| format!("{v} IS NOT NULL"));

        QueryCondition::new(boxed)
    }
}
