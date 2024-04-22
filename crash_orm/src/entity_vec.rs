use crate::{DatabaseConnection, Entity, PrimaryKey};
use async_trait::async_trait;

#[async_trait]
pub trait EntityVec<PRIMARY> {
    /// Shortcut function to call [Entity::persist] on every entity in this vector.
    async fn persist_all(&mut self, connection: &impl DatabaseConnection) -> crate::Result<()>;

    /// Shortcut function to call [Entity::remove] on every entity in this vector.
    async fn remove_all(&mut self, connection: &impl DatabaseConnection) -> crate::Result<()>;
}

#[async_trait]
impl<T: Entity<T, PRIMARY>, PRIMARY: PrimaryKey<'static>> EntityVec<PRIMARY> for Vec<T> {
    async fn persist_all(&mut self, connection: &impl DatabaseConnection) -> crate::Result<()> {
        for entity in self {
            entity.persist(connection).await?;
        }

        Ok(())
    }

    async fn remove_all(&mut self, connection: &impl DatabaseConnection) -> crate::Result<()> {
        for entity in self {
            entity.remove(connection).await?;
        }

        Ok(())
    }
}
