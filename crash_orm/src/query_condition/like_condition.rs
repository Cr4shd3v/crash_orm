use tokio_postgres::types::ToSql;

use crate::{Column, Entity, IntoSql, PrimaryKey, QueryCondition};

/// Trait implementing like operators
pub trait LikeQueryColumn<T: ToSql, U: Entity<U, PRIMARY>, PRIMARY: PrimaryKey> {
    fn like(&self, like: impl IntoSql<String>) -> QueryCondition<U, PRIMARY>;

    fn not_like(&self, like: impl IntoSql<String>) -> QueryCondition<U, PRIMARY>;
}

impl<U: Entity<U, PRIMARY>, R: Column<String, U, PRIMARY>, PRIMARY: PrimaryKey> LikeQueryColumn<String, U, PRIMARY> for R {
    fn like(&self, like: impl IntoSql<String>) -> QueryCondition<U, PRIMARY> {
        QueryCondition::Like(self.get_sql(), like.into_typed_value().get_sql())
    }

    fn not_like(&self, like: impl IntoSql<String>) -> QueryCondition<U, PRIMARY> {
        QueryCondition::NotLike(self.get_sql(), like.into_typed_value().get_sql())
    }
}
