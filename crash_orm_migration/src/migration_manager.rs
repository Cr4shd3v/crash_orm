use chrono::Utc;
use crash_orm::async_trait::async_trait;
use crash_orm::{DatabaseConnection, Entity, EqualQueryColumn};
use crate::{CrashOrmMigrator, Migrator};
use crate::entity::{CrashOrmMigration, CrashOrmMigrationColumn};

#[async_trait]
pub trait CrashOrmMigrationManager<T: DatabaseConnection> {
    fn get_migrations() -> Vec<Box<dyn Migrator<T>>>;

    fn get_all_migrations() -> Vec<Box<dyn Migrator<T>>> {
        let mut migrations = Self::get_migrations();
        migrations.push(Box::new(CrashOrmMigrator));
        migrations
    }

    async fn migrate_up(conn: &T) -> crash_orm::Result<()> {
        let local_migrations = Self::get_all_migrations();

        for local_migration in local_migrations {
            let name = local_migration.get_name().to_string();

            let migration_in_db = CrashOrmMigration::query()
                .condition(CrashOrmMigrationColumn::NAME.equals(&name))
                .execute(conn).await?;

            if migration_in_db.is_empty() {
                local_migration.up(conn).await?;

                let migration_entry = CrashOrmMigration {
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