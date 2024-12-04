use crash_orm::postgres::types::Type;
use crash_orm::prelude::*;
use crash_orm_rocket::CrashOrmDatabaseMigrationFairing;
use crash_orm_test::TEST_DB_URL;

const MIGRATION_TABLE_NAME: &str = "test_rocket_integration";

#[tokio::test]
async fn test_migration_fairing() {
    let rocket = rocket::build()
        .attach(CrashOrmDatabaseMigrationFairing::<MigrationManager>::from_url(TEST_DB_URL))
        .ignite().await.unwrap();
    let conn = rocket.state::<CrashOrmDatabaseConnection>();
    assert!(conn.is_some());
    let conn = conn.unwrap();
    assert_eq!(conn.is_closed(), false);

    TableDefinition::load_from_database(conn, MIGRATION_TABLE_NAME).await.unwrap();
    ExampleMigration.down(conn).await.unwrap();
    assert!(TableDefinition::load_from_database(conn, MIGRATION_TABLE_NAME).await.is_err());
    CrashOrmMigrationRecord::query()
        .condition(CrashOrmMigrationRecordColumn::NAME.equals(MIGRATION_TABLE_NAME))
        .fetch_single(conn).await.unwrap().remove(conn).await.unwrap();
}

struct MigrationManager;

#[async_trait]
impl CrashOrmMigrationManager for MigrationManager
{
    fn get_migrations() -> Vec<Box<dyn Migration>> {
        vec![
            Box::new(ExampleMigration),
        ]
    }
}

struct ExampleMigration;

#[async_trait]
impl Migration for ExampleMigration {
    async fn up(&self, conn: &CrashOrmDatabaseConnection) -> Result<()> {
        TableDefinition::new(MIGRATION_TABLE_NAME)
            .add_column(ColumnDefinition::new("id", Type::INT4, false).primary())?
            .apply(conn).await?;

        Ok(())
    }

    async fn down(&self, conn: &CrashOrmDatabaseConnection) -> Result<()> {
        TableDefinition::drop_table(conn, MIGRATION_TABLE_NAME).await?;

        Ok(())
    }

    fn get_name(&self) -> &str {
        MIGRATION_TABLE_NAME
    }
}
