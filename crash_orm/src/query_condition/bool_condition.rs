use crate::{Column, Entity, PrimaryKey, QueryCondition};
use tokio_postgres::types::ToSql;

/// Trait implementing true/false checks
pub trait BoolQueryColumn<T: ToSql, U: Entity<U, PRIMARY>, PRIMARY: PrimaryKey<'static>> {
    /// Check if this column is TRUE
    fn is_true(&self) -> QueryCondition<U, PRIMARY>;
    /// Check if this column is FALSE
    fn is_false(&self) -> QueryCondition<U, PRIMARY>;
}

impl<U: Entity<U, PRIMARY>, R: Column<bool, U, PRIMARY>, PRIMARY: PrimaryKey<'static>> BoolQueryColumn<bool, U, PRIMARY> for R {
    fn is_true(&self) -> QueryCondition<U, PRIMARY> {
        QueryCondition::IsTrue(self.get_sql())
    }

    fn is_false(&self) -> QueryCondition<U, PRIMARY> {
        QueryCondition::IsFalse(self.get_sql())
    }
}
