use std::marker::PhantomData;
use tokio_postgres::types::ToSql;
use crate::{DatabaseConnection, Entity, QueryCondition};
use crate::entity::slice_query_value_iter;

pub struct EntityColumn<T: ToSql, U: Entity<U> + Send + 'static> {
    pub(crate) name: &'static str,
    phantom_1: PhantomData<T>,
    phantom_2: PhantomData<U>,
}

impl<T: ToSql, U: Entity<U> + Send + 'static> EntityColumn<T, U> {
    pub const fn new(name: &'static str) -> EntityColumn<T, U> {
        Self {
            name,
            phantom_1: PhantomData,
            phantom_2: PhantomData,
        }
    }

    pub fn get_name(&self) -> &str {
        self.name
    }

    pub async fn count(&self, connection: &DatabaseConnection, distinct: bool) -> crate::Result<i64> {
        let row = connection.query_one(
            &*format!("SELECT COUNT({}{}) FROM {}", if distinct { "DISTINCT " } else { "" }, self.name, U::TABLE_NAME),
            &[],
        ).await?;

        Ok(row.get(0))
    }

    pub async fn count_query(&self, connection: &DatabaseConnection, distinct: bool, condition: QueryCondition<U>) -> crate::Result<i64> {
        let (query, values, _) = condition.resolve(1);

        let row = connection.query_one(
            &*format!("SELECT COUNT({}{}) FROM {} WHERE {}", if distinct { "DISTINCT " } else { "" }, self.name, U::TABLE_NAME, query),
            slice_query_value_iter(values.as_slice()).collect::<Vec<&(dyn ToSql + Sync)>>().as_slice(),
        ).await?;

        Ok(row.get(0))
    }
}