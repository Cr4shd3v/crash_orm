use std::ops::Deref;
use tokio_postgres::{Client, Socket};
use tokio_postgres::tls::MakeTlsConnect;
use crate::Entity;

pub struct DatabaseConnection {
    client: Client,
}

impl DatabaseConnection {
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

    /// Shortcut function for [`Entity::persist`]
    pub async fn persist<T: Entity<T>>(&self, entity: &mut T) -> crate::Result<()> {
        entity.persist(self).await
    }

    /// Shortcut function for [`Entity::remove`]
    pub async fn remove<T: Entity<T>>(&self, entity: &mut T) -> crate::Result<()> {
        entity.remove(self).await
    }
}

impl Deref for DatabaseConnection {
    type Target = Client;

    fn deref(&self) -> &Self::Target {
        &self.client
    }
}

#[cfg(test)]
mod tests {
    use crate::DatabaseConnection;

    #[tokio::test]
    async fn test_connection() {
        let connection = DatabaseConnection::test().await;
        assert!(connection.is_ok());
        let connection = connection.unwrap();
        let rows = connection.query_one("SELECT $1::TEXT;", &[&"hello world"]).await;
        assert!(rows.is_ok());
        let rows = rows.unwrap();
        let column: &str = rows.get(0);
        assert_eq!(column, "hello world");
    }
}