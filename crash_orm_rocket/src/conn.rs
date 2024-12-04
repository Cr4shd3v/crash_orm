use crash_orm::connection::CrashOrmDatabaseConnection;
use crash_orm::postgres::NoTls;

pub(crate) async fn init_connection(url: &str) -> CrashOrmDatabaseConnection {
    let conn = CrashOrmDatabaseConnection::new(
        url,
        NoTls,
    ).await.unwrap();

    conn
}