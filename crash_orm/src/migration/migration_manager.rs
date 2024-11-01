use async_trait::async_trait;
use chrono::Utc;

use crate::migration::entity::{CrashOrmMigrationRecord, CrashOrmMigrationRecordColumn};
use crate::migration::migration::Migration;
use crate::prelude::{DatabaseConnection, Entity, EqualQueryColumn, Schema};

/// Trait to be implemented for a migration manager as documented [here](crate::migration).
#[async_trait]
pub trait CrashOrmMigrationManager {
    /// Specifies the migrations for this manager.
    fn get_migrations<T: DatabaseConnection>() -> Vec<Box<dyn Migration<T>>>;

    /// Function used to migrate your database to the latest migration.
    async fn migrate_up(conn: &impl DatabaseConnection) -> crate::Result<()> {
        CrashOrmMigrationRecord::create_table_if_not_exists(conn).await?;

        let local_migrations = Self::get_migrations();

        for local_migration in local_migrations {
            let name = local_migration.get_name().to_string();

            let migration_in_db = CrashOrmMigrationRecord::query()
                .condition(CrashOrmMigrationRecordColumn::NAME.equals(&name))
                .fetch(conn)
                .await?;

            if migration_in_db.is_empty() {
                local_migration.up(conn).await?;

                let mut migration_entry = CrashOrmMigrationRecord {
                    id: None,
                    name,
                    executed_at: Utc::now(),
                };

                migration_entry.insert_set_id(conn).await?;
            }
        }

        Ok(())
    }
}
