use crash_orm::{DatabaseConnection, Schema};
use crash_orm::async_trait::async_trait;
use crate::entity::CrashOrmMigration;

#[async_trait]
pub trait Migrator {
    const NAME: &'static str;

    async fn up(&self, conn: &impl DatabaseConnection) -> crash_orm::Result<()>;

    async fn down(&self, conn: &impl DatabaseConnection) -> crash_orm::Result<()>;

    fn get_name(&self) -> &'static str {
        Self::NAME
    }
}

pub struct CrashOrmMigrator;

#[async_trait]
impl Migrator for CrashOrmMigrator {
    const NAME: &'static str = "CrashOrmBaseMigration";

    async fn up(&self, conn: &impl DatabaseConnection) -> crash_orm::Result<()> {
        CrashOrmMigration::create_table_if_not_exists(conn).await
    }

    async fn down(&self, conn: &impl DatabaseConnection) -> crash_orm::Result<()> {
        CrashOrmMigration::drop_table(conn).await
    }
}