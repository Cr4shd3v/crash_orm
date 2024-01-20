use crash_orm::{DatabaseConnection, Schema};
use crate::entity::CrashOrmMigration;

pub mod entity;

pub struct CrashOrmMigrator;

impl CrashOrmMigrator {
    pub async fn init(db: &impl DatabaseConnection) -> crash_orm::Result<()> {
        if !CrashOrmMigration::table_exists(db).await? {
            CrashOrmMigration::create_table(db).await?;
        }

        Ok(())
    }
}