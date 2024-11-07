use crate::prelude::{BoxedSql, ColumnType, Entity, EntityColumn, VirtualColumn};

/// Trait implementing the count function for a column.
///
/// **NOTE:** This only works on raw [EntityColumn]s! Just as in plain sql.
/// 
/// Also note, that aggregated values like this cannot be used in where statements.
pub trait CountColumn<U: Entity> {
    /// Count function
    fn count_column(&self, distinct: bool) -> VirtualColumn<i64, U>;
}

impl<T: ColumnType, U: Entity> CountColumn<U> for EntityColumn<T, U> {
    fn count_column(&self, distinct: bool) -> VirtualColumn<i64, U> {
        let sql = self.get_sql();
        VirtualColumn::new(BoxedSql::new(
            format!("COUNT({}{})", if distinct { "DISTINCT " } else { "" }, sql.sql),
            sql.values,
        ))
    }
}