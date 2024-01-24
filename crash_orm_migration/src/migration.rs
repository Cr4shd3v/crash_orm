use crate::entity::CrashOrmMigrationRecord;
use crash_orm::async_trait::async_trait;
use crash_orm::{DatabaseConnection, Schema};

#[async_trait]
pub trait Migration<T: DatabaseConnection>: Send + Sync {
    async fn up(&self, conn: &T) -> crash_orm::Result<()>;

    async fn down(&self, conn: &T) -> crash_orm::Result<()>;

    fn get_name(&self) -> String;
}

pub struct CrashOrmBaseMigration;

#[async_trait]
impl<T: DatabaseConnection> Migration<T> for CrashOrmBaseMigration {
    async fn up(&self, conn: &T) -> crash_orm::Result<()> {
        CrashOrmMigrationRecord::create_table_if_not_exists(conn).await
    }

    async fn down(&self, conn: &T) -> crash_orm::Result<()> {
        CrashOrmMigrationRecord::drop_table(conn).await
    }

    fn get_name(&self) -> String {
        String::from("CrashOrmBaseMigration")
    }
}
