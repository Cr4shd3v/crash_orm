use chrono::{DateTime, Utc};

use crash_orm_derive::{Entity, Schema};

use crate as crash_orm;

/// Entity used to track executed migrations
#[derive(Entity, Schema, Debug)]
pub struct CrashOrmMigrationRecord {
    /// ID of the migration
    pub id: u32,
    /// Name of the migration, derived from the [get_name](super::Migration::get_name) method of a migration
    pub name: String,
    /// Execution time of the migration
    pub executed_at: DateTime<Utc>,
}
