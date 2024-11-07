use crate::prelude::{Column, ColumnType, Entity, QueryCondition};

/// Trait implementing null check [QueryCondition].
pub trait NullQueryColumn<T: ColumnType, U: Entity> {
    /// Creates [QueryCondition::IsNull] for self
    fn is_null(&self) -> QueryCondition<U>;

    /// Creates [QueryCondition::IsNotNull] for self
    fn is_not_null(&self) -> QueryCondition<U>;
}

impl<T: ColumnType, U: Entity, C: Column<Option<T>, U>> NullQueryColumn<T, U> for C {
    fn is_null(&self) -> QueryCondition<U> {
        let mut boxed = self.get_sql();
        boxed.modify(|v| format!("{v} IS NULL"));

        QueryCondition::new(boxed)
    }

    fn is_not_null(&self) -> QueryCondition<U> {
        let mut boxed = self.get_sql();
        boxed.modify(|v| format!("{v} IS NOT NULL"));

        QueryCondition::new(boxed)
    }
}
