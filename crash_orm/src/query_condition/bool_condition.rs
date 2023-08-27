use tokio_postgres::types::ToSql;
use crate::{Entity, EntityColumn, QueryCondition};

pub trait BoolQueryColumn<T: ToSql, U: Entity<U> + Send + 'static> {
    fn is_true(&self) -> QueryCondition<U>;
    fn is_false(&self) -> QueryCondition<U>;
}

impl<U: Entity<U> + Send + 'static> BoolQueryColumn<bool, U> for EntityColumn<bool, U> {
    fn is_true(&self) -> QueryCondition<U> {
        QueryCondition::IsTrue(self.name.to_string())
    }

    fn is_false(&self) -> QueryCondition<U> {
        QueryCondition::IsFalse(self.name.to_string())
    }
}

impl<U: Entity<U> + Send + 'static> BoolQueryColumn<bool, U> for EntityColumn<Option<bool>, U> {
    fn is_true(&self) -> QueryCondition<U> {
        QueryCondition::IsTrue(self.name.to_string())
    }

    fn is_false(&self) -> QueryCondition<U> {
        QueryCondition::IsFalse(self.name.to_string())
    }
}