use crash_orm::{DatabaseConnection, Schema};
use crash_orm::async_trait::async_trait;
use crate::entity::CrashOrmMigration;

#[async_trait]
pub trait Migrator<T: DatabaseConnection>: Send + Sync {
    async fn up(&self, conn: &T) -> crash_orm::Result<()>;

    async fn down(&self, conn: &T) -> crash_orm::Result<()>;

    fn get_name(&self) -> String;
}

pub struct CrashOrmMigrator;

#[async_trait]
impl<T: DatabaseConnection> Migrator<T> for CrashOrmMigrator {
    async fn up(&self, conn: &T) -> crash_orm::Result<()> {
        CrashOrmMigration::create_table_if_not_exists(conn).await
    }

    async fn down(&self, conn: &T) -> crash_orm::Result<()> {
        CrashOrmMigration::drop_table(conn).await
    }

    fn get_name(&self) -> String {
        String::from("CrashOrmBaseMigration")
    }
}