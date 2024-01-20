use crash_orm::DatabaseConnection;

pub mod entity;

pub struct CrashOrmMigrator;

impl CrashOrmMigrator {
    pub fn init(db: &impl DatabaseConnection) {

    }
}