use tokio_postgres::types::ToSql;
use crate::{Column, Entity, QueryCondition};

pub trait LikeQueryColumn<T: ToSql, U: Entity<U>> {
    fn like(&self, like: String) -> QueryCondition<U>;

    fn not_like(&self, like: String) -> QueryCondition<U>;
}

impl<U: Entity<U>, R: Column<String, U>> LikeQueryColumn<String, U> for R {
    fn like(&self, like: String) -> QueryCondition<U> {
        QueryCondition::Like(self.get_name(), like)
    }

    fn not_like(&self, like: String) -> QueryCondition<U> {
        QueryCondition::NotLike(self.get_name(), like)
    }
}