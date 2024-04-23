use async_trait::async_trait;

use crate::{DatabaseConnection, Entity, PrimaryKey};

/// Trait implementing useful functions for vectors of entities
#[async_trait]
pub trait EntityVec<P> {
    /// Shortcut function to call [Entity::persist] on every entity in this vector.
    ///
    /// This will be a batch insert/update in the future.
    async fn persist_all(&mut self, connection: &impl DatabaseConnection) -> crate::Result<()>;

    /// Shortcut function to call [Entity::remove] on every entity in this vector.
    ///
    /// This will be a batch operation in the future.
    async fn remove_all(&mut self, connection: &impl DatabaseConnection) -> crate::Result<()>;
}

#[async_trait]
impl<T: Entity<T, P>, P: PrimaryKey> EntityVec<P> for Vec<T> {
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
