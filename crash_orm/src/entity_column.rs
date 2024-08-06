//! Contains the definition of a column of an entity.
//!
//! These columns will be auto generated by the derive macro, so you shouldn't need to use them manually.

use std::marker::PhantomData;

use tokio_postgres::types::ToSql;

pub use avg_column::*;
pub use max_column::*;
pub use min_column::*;
pub use sum_column::*;

use crate::entity::slice_query_value_iter;
use crate::prelude::{BoxedSql, DatabaseConnection, Entity, PrimaryKey, QueryCondition};

mod sum_column;
mod min_column;
mod max_column;
mod avg_column;

/// Struct holding information about a column of an entity.
///
/// These columns will be automatically generated by the entity derive macro.
pub struct EntityColumn<T: ToSql, U: Entity<U, P>, P: PrimaryKey> {
    name: &'static str,
    phantom_1: PhantomData<T>,
    phantom_2: PhantomData<U>,
    phantom_3: PhantomData<P>,
}

impl<T: ToSql, U: Entity<U, P>, P: PrimaryKey> EntityColumn<T, U, P> {
    /// Creates an [EntityColumn] for a column name.
    ///
    /// INTERNAL USE ONLY!
    #[doc(hidden)]
    pub const fn new(name: &'static str) -> EntityColumn<T, U, P> {
        Self {
            name,
            phantom_1: PhantomData,
            phantom_2: PhantomData,
            phantom_3: PhantomData,
        }
    }

    /// Convert [EntityColumn] into a [BoxedSql]
    pub(crate) fn get_sql(&self) -> BoxedSql {
        BoxedSql::new(self.name.to_string(), vec![])
    }

    /// Count entries in this column.
    ///
    /// `distinct`: Only count unique entries. Duplicates are ignored.
    pub async fn count(
        &self,
        connection: &impl DatabaseConnection,
        distinct: bool,
    ) -> crate::Result<i64> {
        let row = connection
            .query_single(
                &*format!(
                    "SELECT COUNT({}{}) FROM {}",
                    if distinct { "DISTINCT " } else { "" },
                    self.name.to_string(),
                    U::TABLE_NAME
                ),
                &[],
            )
            .await?;

        Ok(row.get(0))
    }

    /// Count entries in this column based on a condition.
    ///
    /// `distinct`: Only count unique entries. Duplicates are ignored.
    pub async fn count_query(
        &self,
        connection: &impl DatabaseConnection,
        distinct: bool,
        condition: QueryCondition<U, P>,
    ) -> crate::Result<i64> {
        let (query, values, _) = condition.resolve(1);

        let row = connection
            .query_single(
                &*format!(
                    "SELECT COUNT({}{}) FROM {} WHERE {}",
                    if distinct { "DISTINCT " } else { "" },
                    self.name.to_string(),
                    U::TABLE_NAME,
                    query
                ),
                slice_query_value_iter(values.as_slice())
                    .collect::<Vec<&(dyn ToSql + Sync)>>()
                    .as_slice(),
            )
            .await?;

        Ok(row.get(0))
    }
}
