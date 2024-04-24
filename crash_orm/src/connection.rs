use std::env;
use std::ops::Deref;
use std::sync::Arc;

use tokio_postgres::{Client, Row, Socket};
use tokio_postgres::tls::MakeTlsConnect;
use tokio_postgres::types::ToSql;

/// Trait required to be implemented for a connection to be used by the ORM.
///
/// The default implementation that should be used is [CrashOrmDatabaseConnection].
///
/// You can also just use the default tokio-postgres [Client], this trait is implemented for that as well.
#[async_trait::async_trait]
pub trait DatabaseConnection: Sync {
    /// Method used to only retrieve a single row of a query result.
    async fn query_single(
        &self,
        statement: &str,
        params: &[&(dyn ToSql + Sync)],
    ) -> crate::Result<Row>;

    /// Method used to retrieve all rows of a query result.
    async fn query_many(
        &self,
        statement: &str,
        params: &[&(dyn ToSql + Sync)],
    ) -> crate::Result<Vec<Row>>;

    /// Method used to execute a query without returning a row.
    ///
    /// However, this function returns the count of modified rows in the database.
    async fn execute_query(
        &self,
        statement: &str,
        params: &[&(dyn ToSql + Sync)],
    ) -> crate::Result<u64>;
}

/// The default, simple implementation of the [DatabaseConnection] trait.
pub struct CrashOrmDatabaseConnection {
    client: Client,
}

impl CrashOrmDatabaseConnection {
    /// Creates a new database connection with a connection string and tls
    pub async fn new<T>(config: &str, tls: T) -> crate::Result<Self>
    where
        T: MakeTlsConnect<Socket>,
        <T as MakeTlsConnect<Socket>>::Stream: Send + 'static,
    {
        let (client, connection) = tokio_postgres::connect(config, tls).await?;

        tokio::spawn(async move {
            if let Err(e) = connection.await {
                eprintln!("connection error: {}", e);
            }
        });

        Ok(Self { client })
    }

    #[cfg(test)]
    /// Internal testing only
    pub async fn test() -> crate::Result<Self> {
        Self::new(
            &*env::var("DATABASE_URL").unwrap_or(String::from("postgresql://crash_orm:postgres@localhost/crash_orm_test")),
            tokio_postgres::NoTls,
        )
        .await
    }
}

impl Deref for CrashOrmDatabaseConnection {
    type Target = Client;

    fn deref(&self) -> &Self::Target {
        &self.client
    }
}

macro_rules! impl_database_connection {
    ($class:ty) => {
        #[async_trait::async_trait]
        impl DatabaseConnection for $class {
            async fn query_single(
                &self,
                statement: &str,
                params: &[&(dyn ToSql + Sync)],
            ) -> crate::Result<Row> {
                self.query_one(statement, params)
                    .await
                    .map_err(|e| e.into())
            }

            async fn query_many(
                &self,
                statement: &str,
                params: &[&(dyn ToSql + Sync)],
            ) -> crate::Result<Vec<Row>> {
                self.query(statement, params).await.map_err(|e| e.into())
            }

            async fn execute_query(
                &self,
                statement: &str,
                params: &[&(dyn ToSql + Sync)],
            ) -> crate::Result<u64> {
                self.execute(statement, params).await.map_err(|e| e.into())
            }
        }
    };
}

impl_database_connection!(CrashOrmDatabaseConnection);
impl_database_connection!(Client);

#[async_trait::async_trait]
impl<T: DatabaseConnection + Send> DatabaseConnection for Arc<T> {
    async fn query_single(
        &self,
        statement: &str,
        params: &[&(dyn ToSql + Sync)],
    ) -> crate::Result<Row> {
        self.deref().query_single(statement, params).await
    }

    async fn query_many(
        &self,
        statement: &str,
        params: &[&(dyn ToSql + Sync)],
    ) -> crate::Result<Vec<Row>> {
        self.deref().query_many(statement, params).await
    }

    async fn execute_query(
        &self,
        statement: &str,
        params: &[&(dyn ToSql + Sync)],
    ) -> crate::Result<u64> {
        self.deref().execute_query(statement, params).await
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
        let rows = connection
            .query_one("SELECT $1::TEXT;", &[&"hello world"])
            .await;
        assert!(rows.is_ok());
        let rows = rows.unwrap();
        let column: &str = rows.get(0);
        assert_eq!(column, "hello world");
    }
}
