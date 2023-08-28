use std::ops::Deref;
use tokio_postgres::Client;
use crate::Entity;

pub struct DatabaseConnection {
    client: Client,
}

impl DatabaseConnection {
    pub async fn new(config: &str) -> crate::Result<Self> {
        let (client, connection) =
            tokio_postgres::connect(config, tokio_postgres::NoTls).await?;

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
        Self::new("postgresql://crash_orm:postgres@localhost/crash_orm_test").await
    }

    pub async fn persist<T: Entity<T>>(&self, entity: &mut T) -> crate::Result<()> {
        entity.persist(self).await
    }

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