use async_trait::async_trait;
use tokio_postgres::types::ToSql;

use crate::prelude::{EntityColumn, slice_query_value_iter};
use crate::prelude::{DatabaseConnection, Entity, QueryCondition};

/// Trait implementing the sum functions for columns
#[async_trait]
pub trait SumColumn<T: ToSql, R: ToSql, U: Entity<U>> {
    /// Return the sum of this column
    async fn sum(&self, connection: &impl DatabaseConnection, distinct: bool) -> crate::Result<R>;

    /// Return the sum of this column based on the condition
    async fn sum_query(
        &self,
        connection: &impl DatabaseConnection,
        distinct: bool,
        condition: QueryCondition<U>,
    ) -> crate::Result<R>;
}

macro_rules! impl_sum_column {
    ($in_type:ty, $out_type:ty) => {
        #[async_trait]
        impl<T: Entity<T> + Sync> SumColumn<$in_type, $out_type, T> for EntityColumn<$in_type, T> {
            async fn sum(
                &self,
                connection: &impl DatabaseConnection,
                distinct: bool,
            ) -> crate::Result<$out_type> {
                let (query, values, _) = self.get_sql().resolve(1);

                let row = connection
                    .query_single(
                        &*format!(
                            "SELECT SUM({}{}) FROM {}",
                            if distinct { "DISTINCT " } else { "" },
                            query,
                            T::TABLE_NAME
                        ),
                        slice_query_value_iter(values.as_slice())
                            .collect::<Vec<&(dyn ToSql + Sync)>>()
                            .as_slice(),
                    )
                    .await?;

                Ok(row.get(0))
            }

            async fn sum_query(
                &self,
                connection: &impl DatabaseConnection,
                distinct: bool,
                condition: QueryCondition<T>,
            ) -> crate::Result<$out_type> {
                let (query, mut values, index) = self.get_sql().resolve(1);
                let (con_query, con_values, _) = condition.resolve(index);
                values.extend(con_values);

                let row = connection
                    .query_single(
                        &*format!(
                            "SELECT SUM({}{}) FROM {} WHERE {}",
                            if distinct { "DISTINCT " } else { "" },
                            query,
                            T::TABLE_NAME,
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
        impl<T: Entity<T> + Sync> SumColumn<$in_type, $out_type, T>
            for EntityColumn<Option<$in_type>, T>
        {
            async fn sum(
                &self,
                connection: &impl DatabaseConnection,
                distinct: bool,
            ) -> crate::Result<$out_type> {
                let (query, values, _) = self.get_sql().resolve(1);

                let row = connection
                    .query_single(
                        &*format!(
                            "SELECT SUM({}{}) FROM {}",
                            if distinct { "DISTINCT " } else { "" },
                            query,
                            T::TABLE_NAME
                        ),
                        slice_query_value_iter(values.as_slice())
                            .collect::<Vec<&(dyn ToSql + Sync)>>()
                            .as_slice(),
                    )
                    .await?;

                Ok(row.get(0))
            }

            async fn sum_query(
                &self,
                connection: &impl DatabaseConnection,
                distinct: bool,
                condition: QueryCondition<T>,
            ) -> crate::Result<$out_type> {
                let (query, mut values, index) = self.get_sql().resolve(1);
                let (con_query, con_values, _) = condition.resolve(index);
                values.extend(con_values);

                let row = connection
                    .query_single(
                        &*format!(
                            "SELECT SUM({}{}) FROM {} WHERE {}",
                            if distinct { "DISTINCT " } else { "" },
                            query,
                            T::TABLE_NAME,
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

impl_sum_column!(i8, i64);
impl_sum_column!(i16, i64);
impl_sum_column!(i32, i64);
#[cfg(feature = "with-rust-decimal")]
impl_sum_column!(i64, rust_decimal::Decimal);
#[cfg(feature = "with-rust-decimal")]
impl_sum_column!(rust_decimal::Decimal, rust_decimal::Decimal);
impl_sum_column!(f32, f64);
impl_sum_column!(f64, f64);
