use async_trait::async_trait;
use rust_decimal::Decimal;
use tokio_postgres::types::ToSql;
use crate::{DatabaseConnection, Entity, EntityColumn, QueryCondition};
use crate::entity::slice_query_value_iter;

#[async_trait]
pub trait MinColumn<T: ToSql, U: Entity<U> + Send + 'static> {
    async fn min(&self, connection: &DatabaseConnection) -> crate::Result<T>;

    async fn min_query(&self, connection: &DatabaseConnection, condition: QueryCondition<U>) -> crate::Result<T>;
}

macro_rules! impl_min_column {
    ($column_type:ty) => {
        #[async_trait]
        impl<U: Entity<U> + Send + Sync + 'static> MinColumn<$column_type, U> for EntityColumn<$column_type, U> {
            async fn min(&self, connection: &DatabaseConnection) -> crate::Result<$column_type> {
                let row = connection.query_one(
                    &*format!("SELECT MIN({}) FROM {}", self.name, U::TABLE_NAME),
                    &[],
                ).await?;

                Ok(row.get(0))
            }

            async fn min_query(&self, connection: &DatabaseConnection, condition: QueryCondition<U>) -> crate::Result<$column_type> {
                let (query, values, _) = condition.resolve(1);

                let row = connection.query_one(
                    &*format!("SELECT MIN({}) FROM {} WHERE {}", self.name, U::TABLE_NAME, query),
                    slice_query_value_iter(values.as_slice()).collect::<Vec<&(dyn ToSql + Sync)>>().as_slice(),
                ).await?;

                Ok(row.get(0))
            }
        }

        #[async_trait]
        impl<U: Entity<U> + Send + Sync + 'static> MinColumn<Option<$column_type>, U> for EntityColumn<Option<$column_type>, U> {
            async fn min(&self, connection: &DatabaseConnection) -> crate::Result<Option<$column_type>> {
                let row = connection.query_one(
                    &*format!("SELECT MIN({}) FROM {}", self.name, U::TABLE_NAME),
                    &[],
                ).await?;

                Ok(row.get(0))
            }

            async fn min_query(&self, connection: &DatabaseConnection, condition: QueryCondition<U>) -> crate::Result<Option<$column_type>> {
                let (query, values, _) = condition.resolve(1);

                let row = connection.query_one(
                    &*format!("SELECT MIN({}) FROM {} WHERE {}", self.name, U::TABLE_NAME, query),
                    slice_query_value_iter(values.as_slice()).collect::<Vec<&(dyn ToSql + Sync)>>().as_slice(),
                ).await?;

                Ok(row.get(0))
            }
        }
    };
}

impl_min_column!(i8);
impl_min_column!(i16);
impl_min_column!(i32);
impl_min_column!(i64);
impl_min_column!(Decimal);
impl_min_column!(u32);
impl_min_column!(f32);
impl_min_column!(f64);
impl_min_column!(String);