use async_trait::async_trait;
use tokio_postgres::Row;
use tokio_postgres::types::ToSql;
use crate::{DatabaseConnection, EntityColumn, QueryCondition};

#[async_trait]
pub trait Entity<T: Entity<T> + Send + 'static> {
    const TABLE_NAME: &'static str;

    fn load_from_row(row: &Row) -> T;

    async fn get_by_id(connection: &DatabaseConnection, id: u32) -> crate::Result<T>;

    async fn get_all(connection: &DatabaseConnection) -> crate::Result<Vec<T>>;

    async fn count(connection: &DatabaseConnection) -> crate::Result<i64>;

    async fn insert_get_id(&self, connection: &DatabaseConnection) -> crate::Result<u32>;

    async fn insert_set_id(&mut self, connection: &DatabaseConnection) -> crate::Result<()>;

    async fn remove(&mut self, connection: &DatabaseConnection) -> crate::Result<()>;

    async fn update(&self, connection: &DatabaseConnection) -> crate::Result<()>;

    async fn persist(&mut self, connection: &DatabaseConnection) -> crate::Result<()>;

    async fn query(connection: &DatabaseConnection, condition: QueryCondition<T>) -> crate::Result<Vec<T>> {
        let (query, values, _) = condition.resolve(1);

        let rows = connection.query(
            &*format!("SELECT * FROM {} WHERE {}", Self::TABLE_NAME, query),
            slice_iter(values.as_slice()).collect::<Vec<&(dyn ToSql + Sync)>>().as_slice(),
        ).await?;

        Ok(rows.iter().map(|r| Self::load_from_row(r)).collect())
    }

    async fn count_query(connection: &DatabaseConnection, condition: QueryCondition<T>) -> crate::Result<i64> {
        let (query, values, _) = condition.resolve(1);

        let row = connection.query_one(
            &*format!("SELECT COUNT(*) FROM {} WHERE {}", Self::TABLE_NAME, query),
            slice_iter(values.as_slice()).collect::<Vec<&(dyn ToSql + Sync)>>().as_slice(),
        ).await?;

        Ok(row.get(0))
    }

    async fn count_column<U: ToSql + Send>(connection: &DatabaseConnection, column: EntityColumn<U, T>, distinct: bool) -> crate::Result<i64> {
        let row = connection.query_one(
            &*format!("SELECT COUNT({}{}) FROM {}", if distinct { "DISTINCT " } else { "" }, column.name, Self::TABLE_NAME),
            &[],
        ).await?;

        Ok(row.get(0))
    }

    async fn count_column_query<U: ToSql + Send>(connection: &DatabaseConnection, column: EntityColumn<U, T>, distinct: bool, condition: QueryCondition<T>) -> crate::Result<i64> {
        let (query, values, _) = condition.resolve(1);

        let row = connection.query_one(
            &*format!("SELECT COUNT({}{}) FROM {} WHERE {}", if distinct { "DISTINCT " } else { "" }, column.name, Self::TABLE_NAME, query),
            slice_iter(values.as_slice()).collect::<Vec<&(dyn ToSql + Sync)>>().as_slice(),
        ).await?;

        Ok(row.get(0))
    }
}

fn slice_iter<'a>(
    s: &'a [Box<dyn ToSql + Send + Sync>],
) -> impl ExactSizeIterator<Item = &'a (dyn ToSql + Sync)> + 'a {
    s.iter().map(|s| &**s as _)
}