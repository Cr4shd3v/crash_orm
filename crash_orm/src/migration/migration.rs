use async_trait::async_trait;

use crate::DatabaseConnection;

/// Trait implemented for migrations
#[async_trait]
pub trait Migration<T: DatabaseConnection>: Send + Sync {
    /// Migrate the database up
    async fn up(&self, conn: &T) -> crate::Result<()>;

    /// Migrate the database down
    async fn down(&self, conn: &T) -> crate::Result<()>;

    /// Name of the migration.
    fn get_name(&self) -> String;
}