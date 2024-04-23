use tokio_postgres::types::ToSql;

use crate::{Column, Entity, IntoSql, PrimaryKey, QueryCondition};

/// Trait implementing like operators
pub trait LikeQueryColumn<T: ToSql, U: Entity<U, P>, P: PrimaryKey> {
    fn like(&self, like: impl IntoSql<String>) -> QueryCondition<U, P>;

    fn not_like(&self, like: impl IntoSql<String>) -> QueryCondition<U, P>;
}

impl<U: Entity<U, P>, R: Column<String, U, P>, P: PrimaryKey> LikeQueryColumn<String, U, P> for R {
    fn like(&self, like: impl IntoSql<String>) -> QueryCondition<U, P> {
        QueryCondition::Like(self.get_sql(), like.into_typed_value().get_sql())
    }

    fn not_like(&self, like: impl IntoSql<String>) -> QueryCondition<U, P> {
        QueryCondition::NotLike(self.get_sql(), like.into_typed_value().get_sql())
    }
}
