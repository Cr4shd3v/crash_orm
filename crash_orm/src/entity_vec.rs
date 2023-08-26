use async_trait::async_trait;
use crate::{DatabaseConnection, Entity};

#[async_trait]
pub trait EntityVec {
    async fn persist_all(&mut self, connection: &DatabaseConnection) -> crate::Result<()>;

    async fn remove_all(&mut self, connection: &DatabaseConnection) -> crate::Result<()>;
}

#[async_trait]
impl<T> EntityVec for Vec<T> where T: Entity<T> + Send + 'static {
    async fn persist_all(&mut self, connection: &DatabaseConnection) -> crate::Result<()> {
        for entity in self {
            entity.persist(connection).await?;
        }

        Ok(())
    }

    async fn remove_all(&mut self, connection: &DatabaseConnection) -> crate::Result<()> {
        for entity in self {
            entity.remove(connection).await?;
        }

        Ok(())
    }
}