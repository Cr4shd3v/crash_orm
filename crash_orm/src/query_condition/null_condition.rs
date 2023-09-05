use tokio_postgres::types::ToSql;
use crate::{Entity, EntityColumn, QueryCondition, VirtualColumn};

/// Trait implementing null checks
pub trait NullQueryColumn<T: ToSql, U: Entity<U>> {
    fn is_null(&self) -> QueryCondition<U>;

    fn is_not_null(&self) -> QueryCondition<U>;
}

impl<T: ToSql, U: Entity<U>> NullQueryColumn<T, U> for EntityColumn<Option<T>, U>  {
    fn is_null(&self) -> QueryCondition<U> {
        QueryCondition::IsNull(self.get_sql())
    }

    fn is_not_null(&self) -> QueryCondition<U> {
        QueryCondition::IsNotNull(self.get_sql())
    }
}

impl<T: ToSql, U: Entity<U>> NullQueryColumn<T, U> for VirtualColumn<Option<T>, U>  {
    fn is_null(&self) -> QueryCondition<U> {
        QueryCondition::IsNull(self.get_sql())
    }

    fn is_not_null(&self) -> QueryCondition<U> {
        QueryCondition::IsNotNull(self.get_sql())
    }
}