use chrono::{DateTime, Utc};
use crash_orm::crash_orm_derive::{Entity, Schema};

#[derive(Entity, Schema)]
pub struct CrashOrmMigration {
    pub id: Option<u32>,
    pub name: String,
    pub executed_at: DateTime<Utc>,
    pub last_executed_line: i32,
}