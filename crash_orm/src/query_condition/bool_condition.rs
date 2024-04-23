use crate::{Column, Entity, PrimaryKey, QueryCondition};
use tokio_postgres::types::ToSql;

/// Trait implementing true/false checks
pub trait BoolQueryColumn<T: ToSql, U: Entity<U, P>, P: PrimaryKey> {
    /// Check if this column is TRUE
    fn is_true(&self) -> QueryCondition<U, P>;
    /// Check if this column is FALSE
    fn is_false(&self) -> QueryCondition<U, P>;
}

impl<U: Entity<U, P>, R: Column<bool, U, P>, P: PrimaryKey> BoolQueryColumn<bool, U, P> for R {
    fn is_true(&self) -> QueryCondition<U, P> {
        QueryCondition::IsTrue(self.get_sql())
    }

    fn is_false(&self) -> QueryCondition<U, P> {
        QueryCondition::IsFalse(self.get_sql())
    }
}
