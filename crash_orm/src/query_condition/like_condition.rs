use tokio_postgres::types::ToSql;

use crate::prelude::{Column, Entity, IntoSql, QueryCondition};

/// Trait implementing like operator [QueryCondition]
pub trait LikeQueryColumn<T: ToSql, U: Entity> {
    /// Creates [QueryCondition::Like] from self and other
    fn like(&self, like: impl IntoSql<String>) -> QueryCondition<U>;

    /// Creates [QueryCondition::NotLike] from self and other
    fn not_like(&self, like: impl IntoSql<String>) -> QueryCondition<U>;
}

impl<U: Entity, R: Column<String, U>> LikeQueryColumn<String, U> for R {
    fn like(&self, like: impl IntoSql<String>) -> QueryCondition<U> {
        let mut boxed = self.get_sql();
        let other_boxed = like.into_boxed_sql();
        boxed.modify(|v| format!("{v} LIKE {}", other_boxed.sql));
        boxed.values.extend(other_boxed.values);

        QueryCondition::new(boxed)
    }

    fn not_like(&self, like: impl IntoSql<String>) -> QueryCondition<U> {
        let mut boxed = self.get_sql();
        let other_boxed = like.into_boxed_sql();
        boxed.modify(|v| format!("{v} NOT LIKE {}", other_boxed.sql));
        boxed.values.extend(other_boxed.values);

        QueryCondition::new(boxed)
    }
}
