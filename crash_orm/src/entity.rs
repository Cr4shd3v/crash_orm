use std::fmt::Debug;
use std::sync::Arc;
use async_trait::async_trait;
use tokio_postgres::Row;
use tokio_postgres::types::ToSql;
use crate::{BaseColumn, BoxedColumnValue, DatabaseConnection, Query, QueryCondition, SelectQuery, UntypedColumn};

#[async_trait]
pub trait Entity<T: Entity<T>>: Send + Debug + 'static {
    /// Name of the table
    const TABLE_NAME: &'static str;

    type ColumnType: BaseColumn<T>;

    fn get_id(&self) -> Option<u32>;

    /// Parses a [`Row`] into [`T`]
    fn load_from_row(row: &Row) -> T;

    /// Retrieves an entity by its id
    async fn get_by_id(connection: &impl DatabaseConnection, id: u32) -> crate::Result<T>;

    /// Retrieves all entities
    async fn get_all(connection: &impl DatabaseConnection) -> crate::Result<Vec<T>>;

    /// Returns the count of entries in the table
    async fn count(connection: &impl DatabaseConnection) -> crate::Result<i64>;

    /// Insert and returns the id
    ///
    /// This DOES NOT set the id in the entity
    async fn insert_get_id(&self, connection: &impl DatabaseConnection) -> crate::Result<u32>;

    /// Insert and set id
    ///
    /// This DOES set the id in the entity
    async fn insert_set_id(&mut self, connection: &impl DatabaseConnection) -> crate::Result<()>;

    /// Removes the entity from the database
    async fn remove(&mut self, connection: &impl DatabaseConnection) -> crate::Result<()>;

    /// Updates the entity in the database
    async fn update(&self, connection: &impl DatabaseConnection) -> crate::Result<()>;

    /// Persist this entity.
    ///
    /// If the entity is not yet inserted, [`Self::insert_set_id`] is called.
    /// If the entity is already inserted, [`Self::update`] is called.
    async fn persist(&mut self, connection: &impl DatabaseConnection) -> crate::Result<()>;

    /// Creates a [Query] for this Entity.
    ///
    /// See [Query] for more details on how to build a query.
    fn query() -> Query<T> {
        Query::new(BoxedColumnValue::new(format!("SELECT * FROM {}", Self::TABLE_NAME), vec![]))
    }

    /// Count the entries based on a [QueryCondition].
    async fn count_query(connection: &impl DatabaseConnection, condition: QueryCondition<T>) -> crate::Result<i64> {
        let (query, values, _) = condition.resolve(1);

        let row = connection.query_single(
            &*format!("SELECT COUNT(*) FROM {} WHERE {}", Self::TABLE_NAME, query),
            slice_query_value_iter(values.as_slice()).collect::<Vec<&(dyn ToSql + Sync)>>().as_slice(),
        ).await?;

        Ok(row.get(0))
    }

    /// Select specific columns ([crate::EntityColumn] or [crate::VirtualColumn]) from this entity.
    ///
    /// This returns a [SelectQuery]. See [SelectQuery] for more details.
    fn select_query(columns: &[&(dyn UntypedColumn<T>)]) -> SelectQuery<T> {
        let columns = columns.iter().map(|v| v.get_sql()).collect::<Vec<BoxedColumnValue>>();
        let mut query = vec![];
        let mut values = vec![];
        let mut index = 1;

        for column in columns {
            let (new_query, new_values, next_index) =  column.resolve(index);
            query.push(new_query);
            values.extend(new_values);
            index = next_index;
        }

        SelectQuery::new(BoxedColumnValue::new(format!("SELECT {} FROM {}", query.join(","), Self::TABLE_NAME), values))
    }
}

pub(crate) fn slice_query_value_iter<'a>(
    s: &'a [Arc<Box<dyn ToSql + Send + Sync>>],
) -> impl ExactSizeIterator<Item = &'a (dyn ToSql + Sync)> + 'a {
    s.iter().map(|s| &***s as _)
}