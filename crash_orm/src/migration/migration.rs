use async_trait::async_trait;

use crate::prelude::CrashOrmDatabaseConnection;

/// Trait implemented for migrations
#[async_trait]
pub trait Migration: Send + Sync {
    /// Migrate the database up
    async fn up(&self, conn: &CrashOrmDatabaseConnection) -> crate::Result<()>;

    /// Migrate the database down
    async fn down(&self, conn: &CrashOrmDatabaseConnection) -> crate::Result<()>;

    /// Name of the migration.
    fn get_name(&self) -> &str;
}