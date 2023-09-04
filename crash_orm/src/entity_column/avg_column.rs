use async_trait::async_trait;
use rust_decimal::Decimal;
use tokio_postgres::types::ToSql;
use crate::{DatabaseConnection, Entity, QueryCondition, EntityColumn, slice_query_value_iter};

#[async_trait]
pub trait AvgColumn<T: ToSql, R: ToSql, U: Entity<U>> {
    async fn avg(&self, connection: &DatabaseConnection, distinct: bool) -> crate::Result<R>;

    async fn avg_query(&self, connection: &DatabaseConnection, distinct: bool, condition: QueryCondition<U>) -> crate::Result<R>;
}

macro_rules! impl_avg_column {
    ($in_type:ty, $out_type:ty) => {
        #[async_trait]
        impl<T: Entity<T> + Sync> AvgColumn<$in_type, $out_type, T> for EntityColumn<$in_type, T> {
            async fn avg(&self, connection: &DatabaseConnection, distinct: bool) -> crate::Result<$out_type> {
                let (query, values, _) = self.get_sql().resolve(1);

                let row = connection.query_one(
                    &*format!("SELECT AVG({}{}) FROM {}", if distinct { "DISTINCT " } else { "" }, query, T::TABLE_NAME),
                    slice_query_value_iter(values.as_slice()).collect::<Vec<&(dyn ToSql + Sync)>>().as_slice(),
                ).await?;

                Ok(row.get(0))
            }

            async fn avg_query(&self, connection: &DatabaseConnection, distinct: bool, condition: QueryCondition<T>) -> crate::Result<$out_type> {
                let (query, mut values, index) = self.get_sql().resolve(1);
                let (con_query, con_values, _) = condition.resolve(index);
                values.extend(con_values);

                let row = connection.query_one(
                    &*format!("SELECT AVG({}{}) FROM {} WHERE {}", if distinct { "DISTINCT " } else { "" }, query, T::TABLE_NAME, con_query),
                    slice_query_value_iter(values.as_slice()).collect::<Vec<&(dyn ToSql + Sync)>>().as_slice(),
                ).await?;

                Ok(row.get(0))
            }
        }

        #[async_trait]
        impl<T: Entity<T> + Sync> AvgColumn<$in_type, $out_type, T> for EntityColumn<Option<$in_type>, T> {
            async fn avg(&self, connection: &DatabaseConnection, distinct: bool) -> crate::Result<$out_type> {
                let (query, values, _) = self.get_sql().resolve(1);

                let row = connection.query_one(
                    &*format!("SELECT AVG({}{}) FROM {}", if distinct { "DISTINCT " } else { "" }, query, T::TABLE_NAME),
                    slice_query_value_iter(values.as_slice()).collect::<Vec<&(dyn ToSql + Sync)>>().as_slice(),
                ).await?;

                Ok(row.get(0))
            }

            async fn avg_query(&self, connection: &DatabaseConnection, distinct: bool, condition: QueryCondition<T>) -> crate::Result<$out_type> {
                let (query, mut values, index) = self.get_sql().resolve(1);
                let (con_query, con_values, _) = condition.resolve(index);
                values.extend(con_values);

                let row = connection.query_one(
                    &*format!("SELECT AVG({}{}) FROM {} WHERE {}", if distinct { "DISTINCT " } else { "" }, query, T::TABLE_NAME, con_query),
                    slice_query_value_iter(values.as_slice()).collect::<Vec<&(dyn ToSql + Sync)>>().as_slice(),
                ).await?;

                Ok(row.get(0))
            }
        }
    };
}

impl_avg_column!(i8, Decimal);
impl_avg_column!(i16, Decimal);
impl_avg_column!(i32, Decimal);
impl_avg_column!(i64, Decimal);
impl_avg_column!(Decimal, Decimal);
impl_avg_column!(f32, f64);
impl_avg_column!(f64, f64);