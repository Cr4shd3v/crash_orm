use async_trait::async_trait;
use tokio_postgres::Row;
use crate::DatabaseConnection;

#[async_trait]
pub trait Entity {
    type Output;

    fn load_from_row(row: &Row) -> Self::Output;

    async fn get_by_id(connection: &DatabaseConnection, id: u32) -> Result<Self::Output, tokio_postgres::Error>;

    async fn get_all(connection: &DatabaseConnection) -> Result<Vec<Self::Output>, tokio_postgres::Error>;

    async fn insert_get_id(&self, connection: &DatabaseConnection) -> Result<u32, tokio_postgres::Error>;

    async fn insert_set_id(&mut self, connection: &DatabaseConnection) -> Result<(), tokio_postgres::Error>;

    async fn remove(&mut self, connection: &DatabaseConnection) -> Result<(), tokio_postgres::Error>;

    async fn update(&self, connection: &DatabaseConnection) -> Result<(), tokio_postgres::Error>;

    async fn persist(&mut self, connection: &DatabaseConnection) -> Result<(), tokio_postgres::Error>;
}