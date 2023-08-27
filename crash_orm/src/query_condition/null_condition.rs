use tokio_postgres::types::ToSql;
use crate::{Entity, EntityColumn, QueryCondition, VirtualColumn};

pub trait NullQueryColumn<T: ToSql, U: Entity<U> + Send + 'static> {
    fn is_null(&self) -> QueryCondition<U>;

    fn is_not_null(&self) -> QueryCondition<U>;
}

impl<T: ToSql, U: Entity<U> + Send + 'static> NullQueryColumn<T, U> for EntityColumn<Option<T>, U>  {
    fn is_null(&self) -> QueryCondition<U> {
        QueryCondition::IsNull(self.get_name())
    }

    fn is_not_null(&self) -> QueryCondition<U> {
        QueryCondition::IsNotNull(self.get_name())
    }
}

impl<T: ToSql, U: Entity<U> + Send + 'static> NullQueryColumn<T, U> for VirtualColumn<Option<T>, U>  {
    fn is_null(&self) -> QueryCondition<U> {
        QueryCondition::IsNull(self.get_name())
    }

    fn is_not_null(&self) -> QueryCondition<U> {
        QueryCondition::IsNotNull(self.get_name())
    }
}