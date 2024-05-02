use async_trait::async_trait;
use tokio_postgres::types::ToSql;

use crate::{DatabaseConnection, Entity, EntityColumn, PrimaryKey, QueryCondition, slice_query_value_iter};

/// Trait implementing the avg functions for columns
#[async_trait]
pub trait AvgColumn<T: ToSql, R: ToSql, U: Entity<U, P>, P: PrimaryKey> {
    /// Return the average value of this column
    ///
    /// `distinct`: Only unique entries. Duplicates are ignored.
    async fn avg(&self, connection: &impl DatabaseConnection, distinct: bool) -> crate::Result<R>;

    /// Return the average value of this column based on the condition
    ///
    /// `distinct`: Only unique entries. Duplicates are ignored.
    async fn avg_query(
        &self,
        connection: &impl DatabaseConnection,
        distinct: bool,
        condition: QueryCondition<U, P>,
    ) -> crate::Result<R>;
}

macro_rules! impl_avg_column {
    ($in_type:ty, $out_type:ty) => {
        #[async_trait]
        impl<T: Entity<T, P> + Sync, P: PrimaryKey> AvgColumn<$in_type, $out_type, T, P> for EntityColumn<$in_type, T, P> {
            async fn avg(
                &self,
                connection: &impl DatabaseConnection,
                distinct: bool,
            ) -> crate::Result<$out_type> {
                let (query, values, _) = self.get_sql().resolve(1);

                let row = connection
                    .query_single(
                        &*format!(
                            "SELECT AVG({}{}) FROM {}",
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

            async fn avg_query(
                &self,
                connection: &impl DatabaseConnection,
                distinct: bool,
                condition: QueryCondition<T, P>,
            ) -> crate::Result<$out_type> {
                let (query, mut values, index) = self.get_sql().resolve(1);
                let (con_query, con_values, _) = condition.resolve(index);
                values.extend(con_values);

                let row = connection
                    .query_single(
                        &*format!(
                            "SELECT AVG({}{}) FROM {} WHERE {}",
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
        impl<T: Entity<T, P> + Sync, P: PrimaryKey> AvgColumn<$in_type, $out_type, T, P>
            for EntityColumn<Option<$in_type>, T, P>
        {
            async fn avg(
                &self,
                connection: &impl DatabaseConnection,
                distinct: bool,
            ) -> crate::Result<$out_type> {
                let (query, values, _) = self.get_sql().resolve(1);

                let row = connection
                    .query_single(
                        &*format!(
                            "SELECT AVG({}{}) FROM {}",
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

            async fn avg_query(
                &self,
                connection: &impl DatabaseConnection,
                distinct: bool,
                condition: QueryCondition<T, P>,
            ) -> crate::Result<$out_type> {
                let (query, mut values, index) = self.get_sql().resolve(1);
                let (con_query, con_values, _) = condition.resolve(index);
                values.extend(con_values);

                let row = connection
                    .query_single(
                        &*format!(
                            "SELECT AVG({}{}) FROM {} WHERE {}",
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

#[cfg(feature = "with-rust-decimal")]
impl_avg_column!(i8, rust_decimal::Decimal);
#[cfg(feature = "with-rust-decimal")]
impl_avg_column!(i16, rust_decimal::Decimal);
#[cfg(feature = "with-rust-decimal")]
impl_avg_column!(i32, rust_decimal::Decimal);
#[cfg(feature = "with-rust-decimal")]
impl_avg_column!(i64, rust_decimal::Decimal);
#[cfg(feature = "with-rust-decimal")]
impl_avg_column!(rust_decimal::Decimal, rust_decimal::Decimal);
impl_avg_column!(f32, f64);
impl_avg_column!(f64, f64);
