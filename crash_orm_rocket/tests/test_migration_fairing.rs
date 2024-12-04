use crash_orm::postgres::types::Type;
use crash_orm::prelude::*;
use crash_orm_rocket::CrashOrmDatabaseMigrationFairing;
use crash_orm_test::TEST_DB_URL;

#[tokio::test]
async fn test_migration_fairing() {
    let rocket = rocket::build()
        .attach(CrashOrmDatabaseMigrationFairing::<MigrationManager>::from_url(TEST_DB_URL))
        .ignite().await.unwrap();
    let conn = rocket.state::<CrashOrmDatabaseConnection>();
    assert!(conn.is_some());
    let conn = conn.unwrap();
    assert_eq!(conn.is_closed(), false);
}

struct MigrationManager;

#[async_trait]
impl CrashOrmMigrationManager for MigrationManager
{
    fn get_migrations<T: DatabaseConnection>() -> Vec<Box<dyn Migration<T>>> {
        vec![
            Box::new(ExampleMigration),
        ]
    }
}

struct ExampleMigration;

#[async_trait]
impl Migration<CrashOrmDatabaseConnection> for ExampleMigration {
    async fn up(&self, conn: &CrashOrmDatabaseConnection) -> Result<()> {
        TableDefinition::new("test_rocket_integration")
            .add_column(ColumnDefinition::new("id", Type::INT4, false).primary())?
            .apply(conn).await?;

        Ok(())
    }

    async fn down(&self, conn: &CrashOrmDatabaseConnection) -> Result<()> {
        TableDefinition::drop_table(conn, "test_rocket_integration").await?;

        Ok(())
    }

    fn get_name(&self) -> &str {
        "example"
    }
}
