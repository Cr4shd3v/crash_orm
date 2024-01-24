use crate::entity::{CrashOrmMigrationRecord, CrashOrmMigrationRecordColumn};
use crate::Migration;
use chrono::Utc;
use crash_orm::async_trait::async_trait;
use crash_orm::{DatabaseConnection, Entity, EqualQueryColumn, Schema};

#[async_trait]
pub trait CrashOrmMigrationManager<T: DatabaseConnection> {
    fn get_migrations() -> Vec<Box<dyn Migration<T>>>;

    async fn migrate_up(conn: &T) -> crash_orm::Result<()> {
        CrashOrmMigrationRecord::create_table_if_not_exists(conn).await?;

        let local_migrations = Self::get_migrations();

        for local_migration in local_migrations {
            let name = local_migration.get_name().to_string();

            let migration_in_db = CrashOrmMigrationRecord::query()
                .condition(CrashOrmMigrationRecordColumn::NAME.equals(&name))
                .execute(conn)
                .await?;

            if migration_in_db.is_empty() {
                local_migration.up(conn).await?;

                let migration_entry = CrashOrmMigrationRecord {
                    id: None,
                    name,
                    executed_at: Utc::now(),
                };

                migration_entry.insert_get_id(conn).await?;
            }
        }

        Ok(())
    }
}
