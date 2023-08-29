use tokio_postgres::types::ToSql;
use crate::{Column, Entity, QueryCondition};

pub trait BoolQueryColumn<T: ToSql, U: Entity<U>> {
    fn is_true(&self) -> QueryCondition<U>;
    fn is_false(&self) -> QueryCondition<U>;
}

impl<U: Entity<U> + Send + 'static, R: Column<bool, U>> BoolQueryColumn<bool, U> for R {
    fn is_true(&self) -> QueryCondition<U> {
        QueryCondition::IsTrue(self.get_sql())
    }

    fn is_false(&self) -> QueryCondition<U> {
        QueryCondition::IsFalse(self.get_sql())
    }
}