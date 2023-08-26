use async_trait::async_trait;
use crate::DatabaseConnection;

#[async_trait]
pub trait Schema {
    const TABLE_NAME: &'static str;

    async fn create_table(connection: &DatabaseConnection) -> crate::Result<()>;

    async fn drop_table(connection: &DatabaseConnection) -> crate::Result<()>;

    async fn truncate_table(connection: &DatabaseConnection) -> crate::Result<()>;

    async fn table_exists(connection: &DatabaseConnection) -> crate::Result<bool>;
}