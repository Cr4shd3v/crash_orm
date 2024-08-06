use tokio_postgres::types::ToSql;

use crate::prelude::{Column, Entity, PrimaryKey, QueryCondition};

/// Trait implementing true/false checks
pub trait BoolQueryColumn<T: ToSql, U: Entity<U, P>, P: PrimaryKey> {
    /// Check if this column is TRUE
    fn is_true(&self) -> QueryCondition<U, P>;
    /// Check if this column is FALSE
    fn is_false(&self) -> QueryCondition<U, P>;
}

impl<U: Entity<U, P>, R: Column<bool, U, P>, P: PrimaryKey> BoolQueryColumn<bool, U, P> for R {
    fn is_true(&self) -> QueryCondition<U, P> {
        let mut boxed = self.get_sql();
        boxed.modify(|v| format!("{v} IS TRUE"));

        QueryCondition::new(boxed)
    }

    fn is_false(&self) -> QueryCondition<U, P> {
        let mut boxed = self.get_sql();
        boxed.modify(|v| format!("{v} IS FALSE"));

        QueryCondition::new(boxed)
    }
}
