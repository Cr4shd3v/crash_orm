//! Contains utility functions for vectors of entities.

use postgres::types::ToSql;

use crate::entity::PrimaryKeyEntity;
use crate::prelude::{ColumnType, CreateEntity, DatabaseConnection};

/// Trait implementing useful functions for vectors of entities.
pub trait EntityVec<P: ColumnType> {
    /// Shortcut function to call [Entity::remove] on every entity in this vector.
    ///
    /// This will be a batch operation in the future.
    fn remove_all(&self, connection: &impl DatabaseConnection) -> impl std::future::Future<Output = crate::Result<()>> + Send;
}

impl<T: PrimaryKeyEntity<P>, P: ColumnType> EntityVec<P> for Vec<T> {
    async fn remove_all(&self, connection: &impl DatabaseConnection) -> crate::Result<()> {
        if self.is_empty() {
            return Ok(());
        }

        let ids = self.into_iter()
            .map(|v| v.get_primary())
            .collect::<Vec<P>>();

        let query = format!(
            "DELETE FROM {} WHERE id IN ({})",
            T::TABLE_NAME,
            (0..ids.len()).map(|index| format!("${}", index+1)).collect::<Vec<String>>().join(",")
        );

        connection.execute_query(&*query, slice_to_sql_iter(ids.as_slice())
            .collect::<Vec<&(dyn ToSql + Sync)>>()
            .as_slice()
        ).await?;

        Ok(())
    }
}

/// Trait implementing useful functions for vectors of create entities.
pub trait EntityCreateVec<T: PrimaryKeyEntity<P>, P: ColumnType> {
    /// Batch insert all entities in the vector
    ///
    /// This does **not** update the ids of the entity if needed.
    fn insert_all(self, connection: &impl DatabaseConnection) -> impl std::future::Future<Output = crate::Result<()>> + Send;
}

impl<C: CreateEntity<T>, T: PrimaryKeyEntity<P>, P: ColumnType> EntityCreateVec<T, P> for Vec<C> {
    async fn insert_all(self, connection: &impl DatabaseConnection) -> crate::Result<()> {
        if self.is_empty() {
            return Ok(());
        }

        let insert_field_count = T::__INSERT_FIELD_NAMES.split(",").count();
        let insert_values_string = (0..self.len()).map(|row_index| {
            format!("({})", (0..insert_field_count).map(|value_index| {
                format!("${}", (row_index * insert_field_count) + value_index + 1)
            }).collect::<Vec<String>>().join(","))
        }).collect::<Vec<String>>().join(",");

        let transformed = self.into_iter().map(CreateEntity::into_entity).collect::<Vec<T>>();
        let values = transformed.iter().map(|entity| entity.get_values()).flatten().collect::<Vec<&(dyn ToSql + Sync)>>();

        let query = format!("INSERT INTO {}({}) VALUES {}", T::TABLE_NAME, T::__INSERT_FIELD_NAMES, insert_values_string);
        connection.execute_query(&*query, values.as_slice()).await?;

        Ok(())
    }
}

pub(crate) fn slice_to_sql_iter<'a, T: ToSql + Sync>(
    s: &'a [T],
) -> impl ExactSizeIterator<Item = &'a (dyn ToSql + Sync)> + 'a {
    s.iter().map(|s| s as _)
}