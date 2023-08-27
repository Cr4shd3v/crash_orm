use std::marker::PhantomData;
use tokio_postgres::types::ToSql;
use crate::{DatabaseConnection, Entity, QueryCondition};
use crate::entity::slice_query_value_iter;

pub mod sum_column;
pub use sum_column::*;

mod min_column;
pub use min_column::*;

mod max_column;
pub use max_column::*;

mod avg_column;
pub use avg_column::*;

pub struct EntityColumn<'a, T: ToSql, U: Entity<U> + Send + 'static> {
    name_str: Option<&'a str>,
    name_string: Option<String>,
    phantom_1: PhantomData<T>,
    phantom_2: PhantomData<U>,
}

impl<'a, T: ToSql, U: Entity<U> + Send + 'static> EntityColumn<'a, T, U> {
    pub const fn new(name: &'a str) -> EntityColumn<T, U> {
        Self {
            name_str: Some(name),
            name_string: None,
            phantom_1: PhantomData,
            phantom_2: PhantomData,
        }
    }

    pub fn from_string(name: String) -> EntityColumn<'a, T, U> {
        Self {
            name_str: None,
            name_string: Some(name),
            phantom_1: PhantomData,
            phantom_2: PhantomData,
        }
    }

    pub fn get_name(&self) -> String {
        if self.name_string.is_some() {
            self.name_string.clone().unwrap()
        } else {
            self.name_str.clone().unwrap().to_string()
        }
    }

    pub async fn count(&self, connection: &DatabaseConnection, distinct: bool) -> crate::Result<i64> {
        let row = connection.query_one(
            &*format!("SELECT COUNT({}{}) FROM {}", if distinct { "DISTINCT " } else { "" }, self.get_name(), U::TABLE_NAME),
            &[],
        ).await?;

        Ok(row.get(0))
    }

    pub async fn count_query(&self, connection: &DatabaseConnection, distinct: bool, condition: QueryCondition<U>) -> crate::Result<i64> {
        let (query, values, _) = condition.resolve(1);

        let row = connection.query_one(
            &*format!("SELECT COUNT({}{}) FROM {} WHERE {}", if distinct { "DISTINCT " } else { "" }, self.get_name(), U::TABLE_NAME, query),
            slice_query_value_iter(values.as_slice()).collect::<Vec<&(dyn ToSql + Sync)>>().as_slice(),
        ).await?;

        Ok(row.get(0))
    }
}
