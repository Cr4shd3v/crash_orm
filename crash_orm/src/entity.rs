use async_trait::async_trait;
use tokio_postgres::Row;
use crate::DatabaseConnection;

#[async_trait]
pub trait Entity {
    const TABLE_NAME: &'static str;

    type Output;

    fn load_from_row(row: &Row) -> Self::Output;

    async fn get_by_id(connection: &DatabaseConnection, id: u32) -> crate::Result<Self::Output>;

    async fn get_all(connection: &DatabaseConnection) -> crate::Result<Vec<Self::Output>>;

    async fn insert_get_id(&self, connection: &DatabaseConnection) -> crate::Result<u32>;

    async fn insert_set_id(&mut self, connection: &DatabaseConnection) -> crate::Result<()>;

    async fn remove(&mut self, connection: &DatabaseConnection) -> crate::Result<()>;

    async fn update(&self, connection: &DatabaseConnection) -> crate::Result<()>;

    async fn persist(&mut self, connection: &DatabaseConnection) -> crate::Result<()>;
}