use crash_orm::CrashOrmDatabaseConnection;
use crash_orm::postgres::NoTls;

pub async fn setup_test_connection() -> CrashOrmDatabaseConnection {
    CrashOrmDatabaseConnection::new(
        "postgresql://crash_orm:postgres@localhost/crash_orm_test",
        NoTls,
    ).await.unwrap()
}