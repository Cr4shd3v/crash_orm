use tokio_postgres::types::ToSql;

use crate::{Column, Entity, IntoSql, QueryCondition};

/// Trait implementing like operators
pub trait LikeQueryColumn<T: ToSql, U: Entity<U>> {
    fn like(&self, like: impl IntoSql<String>) -> QueryCondition<U>;

    fn not_like(&self, like: impl IntoSql<String>) -> QueryCondition<U>;
}

impl<U: Entity<U>, R: Column<String, U>> LikeQueryColumn<String, U> for R {
    fn like(&self, like: impl IntoSql<String>) -> QueryCondition<U> {
        QueryCondition::Like(self.get_sql(), like.into_typed_value().get_sql())
    }

    fn not_like(&self, like: impl IntoSql<String>) -> QueryCondition<U> {
        QueryCondition::NotLike(self.get_sql(), like.into_typed_value().get_sql())
    }
}
