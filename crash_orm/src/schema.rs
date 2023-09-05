use async_trait::async_trait;
use crate::DatabaseConnection;

#[async_trait]
pub trait Schema {
    /// Create the table based on the provided struct.
    async fn create_table(connection: &DatabaseConnection) -> crate::Result<()>;

    /// Drop the table
    async fn drop_table(connection: &DatabaseConnection) -> crate::Result<()>;

    /// Empty the table
    async fn truncate_table(connection: &DatabaseConnection) -> crate::Result<()>;

    /// Check whether the table exists or not
    async fn table_exists(connection: &DatabaseConnection) -> crate::Result<bool>;
}