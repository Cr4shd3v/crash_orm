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
        let mut boxed = self.get_sql();
        let other_boxed = like.into_boxed_sql();
        boxed.modify(|v| format!("{v} LIKE {}", other_boxed.sql));
        boxed.values.extend(other_boxed.values);

        QueryCondition::new(boxed)
    }

    fn not_like(&self, like: impl IntoSql<String>) -> QueryCondition<U, P> {
        let mut boxed = self.get_sql();
        let other_boxed = like.into_boxed_sql();
        boxed.modify(|v| format!("{v} NOT LIKE {}", other_boxed.sql));
        boxed.values.extend(other_boxed.values);

        QueryCondition::new(boxed)
    }
}
