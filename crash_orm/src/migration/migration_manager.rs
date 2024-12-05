use async_trait::async_trait;
use chrono::Utc;

use crate::migration::entity::{CrashOrmMigrationRecord, CrashOrmMigrationRecordColumn};
use crate::migration::migration::Migration;
use crate::prelude::{CrashOrmDatabaseConnection, Entity, EqualQueryColumn, OrderDirection, Schema};

/// Trait to be implemented for a migration manager as documented [here](crate::migration).
#[async_trait]
pub trait CrashOrmMigrationManager: Sync + Send + 'static {
    /// Specifies the migrations for this manager.
    fn get_migrations() -> Vec<Box<dyn Migration>>;

    /// Function used to migrate your database to the latest migration.
    async fn migrate_up(conn: &CrashOrmDatabaseConnection) -> crate::Result<()> {
        CrashOrmMigrationRecord::create_table_if_not_exists(conn).await?;

        let local_migrations = Self::get_migrations();

        for local_migration in local_migrations {
            let name = local_migration.get_name();

            let migration_in_db = CrashOrmMigrationRecord::query()
                .condition(CrashOrmMigrationRecordColumn::NAME.equals(name))
                .fetch(conn)
                .await?;

            if migration_in_db.is_empty() {
                local_migration.up(conn).await?;

                let mut migration_entry = CrashOrmMigrationRecord {
                    id: None,
                    name: name.to_string(),
                    executed_at: Utc::now(),
                };

                migration_entry.insert(conn).await?;
            }
        }

        Ok(())
    }

    /// This function migrates your database down to the desired version
    async fn migrate_down_to(conn: &CrashOrmDatabaseConnection, name: &str) -> crate::Result<()> {
        let mut local_migrations = Self::get_migrations();
        local_migrations.reverse();

        let latest = CrashOrmMigrationRecord::query()
            .order(&CrashOrmMigrationRecordColumn::ID, OrderDirection::DESC)
            .fetch_single(conn).await?;

        let mut started = false;
        for migration in local_migrations {
            if !started && migration.get_name() == latest.name {
                started = true;
            }

            if started {
                migration.down(conn).await?;

                CrashOrmMigrationRecord::query()
                    .condition(CrashOrmMigrationRecordColumn::NAME.equals(migration.get_name()))
                    .fetch_single(conn).await?.remove(conn).await?;
            }

            if migration.get_name() == name {
                break;
            }
        }

        Ok(())
    }

    /// Migrate down to the previous migration
    async fn migrate_down_prev(conn: &CrashOrmDatabaseConnection) -> crate::Result<()> {
        let local_migrations = Self::get_migrations();

        let mut latest = CrashOrmMigrationRecord::query()
            .order(&CrashOrmMigrationRecordColumn::ID, OrderDirection::DESC)
            .fetch_single(conn).await?;

        let Some(local_migration) = local_migrations.iter().find(|m| m.get_name() == latest.name) else {
            return Err(crate::Error::from_str(&*format!("The previous migration {} was not found in local migrations", latest.name)));
        };

        local_migration.down(conn).await?;

        latest.remove(conn).await?;

        Ok(())
    }
}
