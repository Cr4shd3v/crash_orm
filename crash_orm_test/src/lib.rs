use crash_orm::prelude::CrashOrmDatabaseConnection;
use std::env;
use tokio_postgres::NoTls;

pub const TEST_DB_URL: &str = "postgresql://crash_orm:postgres@localhost/crash_orm_test";

pub async fn setup_test_connection() -> CrashOrmDatabaseConnection {
    CrashOrmDatabaseConnection::new(
        &*env::var("DATABASE_URL").unwrap_or(String::from(TEST_DB_URL)),
        NoTls,
    ).await.unwrap()
}

#[macro_export]
macro_rules! default_create_table {
    ($entity:ty, $conn:expr) => {
        if !<$entity>::table_exists(&$conn).await.unwrap() {
            <$entity>::create_table(&$conn).await.unwrap();
        } else {
            <$entity>::truncate_table(&$conn).await.unwrap();
        }
    };
}