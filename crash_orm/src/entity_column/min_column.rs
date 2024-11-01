use async_trait::async_trait;
use tokio_postgres::types::ToSql;

use crate::entity::slice_query_value_iter;
use crate::prelude::{DatabaseConnection, Entity, EntityColumn, QueryCondition};

/// Trait implementing the min functions for columns
#[async_trait]
pub trait MinColumn<T: ToSql, U: Entity> {
    /// Return the minimum value of this column
    async fn min(&self, connection: &impl DatabaseConnection) -> crate::Result<T>;

    /// Return the minimum value of this column based on the condition
    async fn min_query(
        &self,
        connection: &impl DatabaseConnection,
        condition: QueryCondition<U>,
    ) -> crate::Result<T>;
}

macro_rules! impl_min_column {
    ($column_type:ty) => {
        #[async_trait]
        impl<U: Entity + Sync> MinColumn<$column_type, U> for EntityColumn<$column_type, U> {
            async fn min(
                &self,
                connection: &impl DatabaseConnection,
            ) -> crate::Result<$column_type> {
                let (query, values, _) = self.get_sql().resolve(1);

                let row = connection
                    .query_single(
                        &*format!("SELECT MIN({}) FROM {}", query, U::TABLE_NAME),
                        slice_query_value_iter(values.as_slice())
                            .collect::<Vec<&(dyn ToSql + Sync)>>()
                            .as_slice(),
                    )
                    .await?;

                Ok(row.get(0))
            }

            async fn min_query(
                &self,
                connection: &impl DatabaseConnection,
                condition: QueryCondition<U>,
            ) -> crate::Result<$column_type> {
                let (query, mut values, index) = self.get_sql().resolve(1);
                let (con_query, con_values, _) = condition.resolve(index);
                values.extend(con_values);

                let row = connection
                    .query_single(
                        &*format!(
                            "SELECT MIN({}) FROM {} WHERE {}",
                            query,
                            U::TABLE_NAME,
                            con_query
                        ),
                        slice_query_value_iter(values.as_slice())
                            .collect::<Vec<&(dyn ToSql + Sync)>>()
                            .as_slice(),
                    )
                    .await?;

                Ok(row.get(0))
            }
        }

        #[async_trait]
        impl<U: Entity + Sync> MinColumn<Option<$column_type>, U>
            for EntityColumn<Option<$column_type>, U>
        {
            async fn min(
                &self,
                connection: &impl DatabaseConnection,
            ) -> crate::Result<Option<$column_type>> {
                let (query, values, _) = self.get_sql().resolve(1);

                let row = connection
                    .query_single(
                        &*format!("SELECT MIN({}) FROM {}", query, U::TABLE_NAME),
                        slice_query_value_iter(values.as_slice())
                            .collect::<Vec<&(dyn ToSql + Sync)>>()
                            .as_slice(),
                    )
                    .await?;

                Ok(row.get(0))
            }

            async fn min_query(
                &self,
                connection: &impl DatabaseConnection,
                condition: QueryCondition<U>,
            ) -> crate::Result<Option<$column_type>> {
                let (query, mut values, index) = self.get_sql().resolve(1);
                let (con_query, con_values, _) = condition.resolve(index);
                values.extend(con_values);

                let row = connection
                    .query_single(
                        &*format!(
                            "SELECT MIN({}) FROM {} WHERE {}",
                            query,
                            U::TABLE_NAME,
                            con_query
                        ),
                        slice_query_value_iter(values.as_slice())
                            .collect::<Vec<&(dyn ToSql + Sync)>>()
                            .as_slice(),
                    )
                    .await?;

                Ok(row.get(0))
            }
        }
    };
}

impl_min_column!(i8);
impl_min_column!(i16);
impl_min_column!(i32);
impl_min_column!(i64);
#[cfg(feature = "with-rust-decimal")]
impl_min_column!(rust_decimal::Decimal);
impl_min_column!(u32);
impl_min_column!(f32);
impl_min_column!(f64);
impl_min_column!(String);
