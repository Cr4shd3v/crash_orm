use tokio_postgres::types::ToSql;

use crate::prelude::{Column, Entity, QueryCondition};

/// Trait implementing true/false checks
pub trait BoolQueryColumn<T: ToSql, U: Entity> {
    /// Check if this column is TRUE
    fn is_true(&self) -> QueryCondition<U>;
    /// Check if this column is FALSE
    fn is_false(&self) -> QueryCondition<U>;
}

impl<U: Entity, R: Column<bool, U>> BoolQueryColumn<bool, U> for R {
    fn is_true(&self) -> QueryCondition<U> {
        let mut boxed = self.get_sql();
        boxed.modify(|v| format!("{v} IS TRUE"));

        QueryCondition::new(boxed)
    }

    fn is_false(&self) -> QueryCondition<U> {
        let mut boxed = self.get_sql();
        boxed.modify(|v| format!("{v} IS FALSE"));

        QueryCondition::new(boxed)
    }
}
