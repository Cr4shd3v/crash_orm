use async_trait::async_trait;
use rust_decimal::Decimal;
use tokio_postgres::types::ToSql;
use crate::{DatabaseConnection, Entity, EntityColumn, QueryCondition};
use crate::entity::slice_query_value_iter;

/// Trait implementing the max functions for columns
#[async_trait]
pub trait MaxColumn<T: ToSql, U: Entity<U>> {
    /// Return the maximum value of this column
    async fn max(&self, connection: &DatabaseConnection) -> crate::Result<T>;

    /// Return the maximum value of this column based on the condition
    async fn max_query(&self, connection: &DatabaseConnection, condition: QueryCondition<U>) -> crate::Result<T>;
}

macro_rules! impl_max_column {
    ($column_type:ty) => {
        #[async_trait]
        impl<U: Entity<U> + Sync> MaxColumn<$column_type, U> for EntityColumn<$column_type, U> {
            async fn max(&self, connection: &DatabaseConnection) -> crate::Result<$column_type> {
                let (query, values, _) = self.get_sql().resolve(1);

                let row = connection.query_one(
                    &*format!("SELECT MAX({}) FROM {}", query, U::TABLE_NAME),
                    slice_query_value_iter(values.as_slice()).collect::<Vec<&(dyn ToSql + Sync)>>().as_slice(),
                ).await?;

                Ok(row.get(0))
            }

            async fn max_query(&self, connection: &DatabaseConnection, condition: QueryCondition<U>) -> crate::Result<$column_type> {
                let (query, mut values, index) = self.get_sql().resolve(1);
                let (con_query, con_values, _) = condition.resolve(index);
                values.extend(con_values);

                let row = connection.query_one(
                    &*format!("SELECT MAX({}) FROM {} WHERE {}", query, U::TABLE_NAME, con_query),
                    slice_query_value_iter(values.as_slice()).collect::<Vec<&(dyn ToSql + Sync)>>().as_slice(),
                ).await?;

                Ok(row.get(0))
            }
        }

        #[async_trait]
        impl<U: Entity<U> + Sync> MaxColumn<Option<$column_type>, U> for EntityColumn<Option<$column_type>, U> {
            async fn max(&self, connection: &DatabaseConnection) -> crate::Result<Option<$column_type>> {
                let (query, values, _) = self.get_sql().resolve(1);

                let row = connection.query_one(
                    &*format!("SELECT MAX({}) FROM {}", query, U::TABLE_NAME),
                    slice_query_value_iter(values.as_slice()).collect::<Vec<&(dyn ToSql + Sync)>>().as_slice(),
                ).await?;

                Ok(row.get(0))
            }

            async fn max_query(&self, connection: &DatabaseConnection, condition: QueryCondition<U>) -> crate::Result<Option<$column_type>> {
                let (query, mut values, index) = self.get_sql().resolve(1);
                let (con_query, con_values, _) = condition.resolve(index);
                values.extend(con_values);

                let row = connection.query_one(
                    &*format!("SELECT MAX({}) FROM {} WHERE {}", query, U::TABLE_NAME, con_query),
                    slice_query_value_iter(values.as_slice()).collect::<Vec<&(dyn ToSql + Sync)>>().as_slice(),
                ).await?;

                Ok(row.get(0))
            }
        }
    };
}

impl_max_column!(i8);
impl_max_column!(i16);
impl_max_column!(i32);
impl_max_column!(i64);
impl_max_column!(Decimal);
impl_max_column!(u32);
impl_max_column!(f32);
impl_max_column!(f64);
impl_max_column!(String);