use tokio_postgres::types::ToSql;
use crate::{Column, Entity, QueryCondition};

/// Trait implementing true/false checks
pub trait BoolQueryColumn<T: ToSql, U: Entity<U>> {
    /// Check if this column is TRUE
    fn is_true(&self) -> QueryCondition<U>;
    /// Check if this column is FALSE
    fn is_false(&self) -> QueryCondition<U>;
}

impl<U: Entity<U> + Send + 'static, R: Column<bool, U>> BoolQueryColumn<bool, U> for R {
    fn is_true(&self) -> QueryCondition<U> {
        QueryCondition::IsTrue(self.get_sql())
    }

    fn is_false(&self) -> QueryCondition<U> {
        QueryCondition::IsFalse(self.get_sql())
    }
}