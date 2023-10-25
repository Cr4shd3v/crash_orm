use std::ops::Deref;
use tokio_postgres::{Client, Row, Socket};
use tokio_postgres::tls::MakeTlsConnect;
use tokio_postgres::types::ToSql;

#[async_trait::async_trait]
pub trait DatabaseConnection: Sync {
    async fn query_single(&self, statement: &str, params: &[&(dyn ToSql + Sync)]) -> crate::Result<Row>;

    async fn query_many(&self, statement: &str, params: &[&(dyn ToSql + Sync)]) -> crate::Result<Vec<Row>>;

    async fn execute_query(&self, statement: &str, params: &[&(dyn ToSql + Sync)]) -> crate::Result<u64>;
}

pub struct CrashOrmDatabaseConnection {
    client: Client,
}

impl CrashOrmDatabaseConnection {
    /// Creates a new database connection with a connection string and tls
    pub async fn new<T>(config: &str, tls: T) -> crate::Result<Self> where T: MakeTlsConnect<Socket>, <T as MakeTlsConnect<Socket>>::Stream: Send + 'static {
        let (client, connection) =
            tokio_postgres::connect(config, tls).await?;

        tokio::spawn(async move {
            if let Err(e) = connection.await {
                eprintln!("connection error: {}", e);
            }
        });

        Ok(Self {
            client,
        })
    }

    #[cfg(test)]
    pub async fn test() -> crate::Result<Self> {
        Self::new("postgresql://crash_orm:postgres@localhost/crash_orm_test", tokio_postgres::NoTls).await
    }
}

#[async_trait::async_trait]
impl DatabaseConnection for CrashOrmDatabaseConnection {
    async fn query_single(&self, statement: &str, params: &[&(dyn ToSql + Sync)]) -> crate::Result<Row> {
        self.query_one(statement, params).await.map_err(|e| e.into())
    }

    async fn query_many(&self, statement: &str, params: &[&(dyn ToSql + Sync)]) -> crate::Result<Vec<Row>> {
        self.query(statement, params).await.map_err(|e| e.into())
    }

    async fn execute_query(&self, statement: &str, params: &[&(dyn ToSql + Sync)]) -> crate::Result<u64> {
        self.execute(statement, params).await.map_err(|e| e.into())
    }
}

impl Deref for CrashOrmDatabaseConnection {
    type Target = Client;

    fn deref(&self) -> &Self::Target {
        &self.client
    }
}

#[cfg(test)]
mod tests {
    use crate::CrashOrmDatabaseConnection;

    #[tokio::test]
    async fn test_connection() {
        let connection = CrashOrmDatabaseConnection::test().await;
        assert!(connection.is_ok());
        let connection = connection.unwrap();
        let rows = connection.query_one("SELECT $1::TEXT;", &[&"hello world"]).await;
        assert!(rows.is_ok());
        let rows = rows.unwrap();
        let column: &str = rows.get(0);
        assert_eq!(column, "hello world");
    }
}