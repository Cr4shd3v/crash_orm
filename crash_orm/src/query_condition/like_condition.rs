use crate::{Column, Entity, QueryCondition, TypedColumnValue};
use tokio_postgres::types::ToSql;

/// Trait implementing like operators
pub trait LikeQueryColumn<T: ToSql, U: Entity<U>> {
    fn like(&self, like: &(dyn TypedColumnValue<String>)) -> QueryCondition<U>;

    fn not_like(&self, like: &(dyn TypedColumnValue<String>)) -> QueryCondition<U>;
}

impl<U: Entity<U>, R: Column<String, U>> LikeQueryColumn<String, U> for R {
    fn like(&self, like: &(dyn TypedColumnValue<String>)) -> QueryCondition<U> {
        QueryCondition::Like(self.get_sql(), like.get_sql())
    }

    fn not_like(&self, like: &(dyn TypedColumnValue<String>)) -> QueryCondition<U> {
        QueryCondition::NotLike(self.get_sql(), like.get_sql())
    }
}
