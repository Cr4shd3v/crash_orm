use tokio_postgres::types::ToSql;

use crate::prelude::{Column, Entity, IntoSql, PrimaryKey, QueryCondition};

/// Trait implementing like operator [QueryCondition]
pub trait LikeQueryColumn<T: ToSql, U: Entity<U, P>, P: PrimaryKey> {
    /// Creates [QueryCondition::Like] from self and other
    fn like(&self, like: impl IntoSql<String>) -> QueryCondition<U, P>;

    /// Creates [QueryCondition::NotLike] from self and other
    fn not_like(&self, like: impl IntoSql<String>) -> QueryCondition<U, P>;
}

impl<U: Entity<U, P>, R: Column<String, U, P>, P: PrimaryKey> LikeQueryColumn<String, U, P> for R {
    fn like(&self, like: impl IntoSql<String>) -> QueryCondition<U, P> {
        QueryCondition::Like(self.get_sql(), like.into_boxed_sql())
    }

    fn not_like(&self, like: impl IntoSql<String>) -> QueryCondition<U, P> {
        QueryCondition::NotLike(self.get_sql(), like.into_boxed_sql())
    }
}
