use async_trait::async_trait;
use rust_decimal::Decimal;
use tokio_postgres::types::ToSql;
use crate::{DatabaseConnection, Entity, QueryCondition};
use crate::{slice_query_value_iter, EntityColumn};

#[async_trait]
pub trait SumColumn<T: ToSql, R: ToSql, U: Entity<U> + Send + 'static> {
    async fn sum(&self, connection: &DatabaseConnection, distinct: bool) -> crate::Result<R>;

    async fn sum_query(&self, connection: &DatabaseConnection, distinct: bool, condition: QueryCondition<U>) -> crate::Result<R>;
}

macro_rules! impl_sum_column {
    ($in_type:ty, $out_type:ty) => {
        #[async_trait]
        impl<T: Entity<T> + Send + Sync + 'static> SumColumn<$in_type, $out_type, T> for EntityColumn<$in_type, T> {
            async fn sum(&self, connection: &DatabaseConnection, distinct: bool) -> crate::Result<$out_type> {
                let row = connection.query_one(
                    &*format!("SELECT SUM({}{}) FROM {}", if distinct { "DISTINCT " } else { "" }, self.name, T::TABLE_NAME),
                    &[],
                ).await?;

                Ok(row.get(0))
            }

            async fn sum_query(&self, connection: &DatabaseConnection, distinct: bool, condition: QueryCondition<T>) -> crate::Result<$out_type> {
                let (query, values, _) = condition.resolve(1);

                let row = connection.query_one(
                    &*format!("SELECT SUM({}{}) FROM {} WHERE {}", if distinct { "DISTINCT " } else { "" }, self.name, T::TABLE_NAME, query),
                    slice_query_value_iter(values.as_slice()).collect::<Vec<&(dyn ToSql + Sync)>>().as_slice(),
                ).await?;

                Ok(row.get(0))
            }
        }

        #[async_trait]
        impl<T: Entity<T> + Send + Sync + 'static> SumColumn<$in_type, $out_type, T> for EntityColumn<Option<$in_type>, T> {
            async fn sum(&self, connection: &DatabaseConnection, distinct: bool) -> crate::Result<$out_type> {
                let row = connection.query_one(
                    &*format!("SELECT SUM({}{}) FROM {}", if distinct { "DISTINCT " } else { "" }, self.name, T::TABLE_NAME),
                    &[],
                ).await?;

                Ok(row.get(0))
            }

            async fn sum_query(&self, connection: &DatabaseConnection, distinct: bool, condition: QueryCondition<T>) -> crate::Result<$out_type> {
                let (query, values, _) = condition.resolve(1);

                let row = connection.query_one(
                    &*format!("SELECT SUM({}{}) FROM {} WHERE {}", if distinct { "DISTINCT " } else { "" }, self.name, T::TABLE_NAME, query),
                    slice_query_value_iter(values.as_slice()).collect::<Vec<&(dyn ToSql + Sync)>>().as_slice(),
                ).await?;

                Ok(row.get(0))
            }
        }
    };
}

impl_sum_column!(i8, i64);
impl_sum_column!(i16, i64);
impl_sum_column!(i32, i64);
impl_sum_column!(i64, Decimal);
impl_sum_column!(Decimal, Decimal);
impl_sum_column!(f32, f64);
impl_sum_column!(f64, f64);
