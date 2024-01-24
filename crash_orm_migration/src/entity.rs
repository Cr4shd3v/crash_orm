use chrono::{DateTime, Utc};
use crash_orm::crash_orm_derive::{Entity, Schema};

#[derive(Entity, Schema, Debug)]
pub struct CrashOrmMigrationRecord {
    pub id: Option<u32>,
    pub name: String,
    pub executed_at: DateTime<Utc>,
}