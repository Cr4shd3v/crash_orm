use crash_orm::CrashOrmDatabaseConnection;
use crash_orm::postgres::NoTls;

pub async fn setup_test_connection() -> CrashOrmDatabaseConnection {
    CrashOrmDatabaseConnection::new(
        "postgresql://crash_orm:postgres@localhost/crash_orm_test",
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