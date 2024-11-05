use crate::prelude::{BoxedSql, Entity, EntityColumn, VirtualColumn};
use postgres::types::ToSql;

/// Trait implementing the count function for a column.
///
/// **NOTE:** This only works on raw [EntityColumn]s! Just as in plain sql.
/// 
/// Also note, that aggregated values like this cannot be used in where statements.
pub trait CountColumn<T: ToSql, U: Entity> {
    /// Count function
    fn count_column(&self, distinct: bool) -> VirtualColumn<T, U>;
}

impl<T: ToSql, U: Entity> CountColumn<T, U> for EntityColumn<T, U> {
    fn count_column(&self, distinct: bool) -> VirtualColumn<T, U> {
        let sql = self.get_sql();
        VirtualColumn::new(BoxedSql::new(
            format!("COUNT({}{})", if distinct { "DISTINCT " } else { "" }, sql.sql),
            sql.values,
        ))
    }
}