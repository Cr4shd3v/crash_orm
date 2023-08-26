use async_trait::async_trait;
use tokio_postgres::Row;
use tokio_postgres::types::ToSql;
use crate::{DatabaseConnection, EntityColumn};

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

    async fn count_column<U: ToSql + Send>(connection: &DatabaseConnection, column: EntityColumn<U, T>, distinct: bool) -> crate::Result<i64> {
        let row = connection.query_one(
            &*format!("SELECT COUNT({}{}) FROM {}", if distinct { "DISTINCT " } else { "" }, column.name, Self::TABLE_NAME),
            &[],
        ).await?;

        Ok(row.get(0))
    }
}