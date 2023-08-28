use tokio_postgres::Row;
use tokio_postgres::types::ToSql;
use crate::{DatabaseConnection, Entity, QueryCondition};
use crate::entity::slice_query_value_iter;

macro_rules! base_query_functions {
    ($base:ident) => {
        pub fn new(base_query: String) -> $base<T> {
            Self {
                base_query,
                condition: None,
            }
        }

        pub fn condition(mut self, condition: QueryCondition<T>) -> $base<T> {
            self.condition = Some(condition);
            self
        }

        pub fn get_raw_query(self) -> (String, Vec<Box<dyn ToSql+Send+Sync>>) {
            let mut query = String::new();
            let mut values: Vec<Box<dyn ToSql+Send+Sync>> = vec![];
            query.push_str(&*self.base_query);

            if self.condition.is_some() {
                let (condition_query, condition_values, _) = self.condition.unwrap().resolve(1);
                values = condition_values;
                query.push_str(" WHERE ");
                query.push_str(&*condition_query);
            }

            (query, values)
        }
    };
}

pub struct Query<T: Entity<T>> {
    base_query: String,
    condition: Option<QueryCondition<T>>,
}

impl<T: Entity<T>> Query<T> {
    base_query_functions!(Query);

    pub async fn execute(self, connection: &DatabaseConnection) -> crate::Result<Vec<T>> {
        let (query, values) = self.get_raw_query();

        let rows = connection.query(
            &*query,
            slice_query_value_iter(values.as_slice()).collect::<Vec<&(dyn ToSql + Sync)>>().as_slice()
        ).await?;

        Ok(rows.iter().map(|r| T::load_from_row(r)).collect::<Vec<T>>())
    }
}

pub struct SelectQuery<T: Entity<T>> {
    base_query: String,
    condition: Option<QueryCondition<T>>,
}

impl<T: Entity<T>> SelectQuery<T> {
    base_query_functions!(SelectQuery);

    pub async fn execute(self, connection: &DatabaseConnection) -> crate::Result<Vec<Row>> {
        let (query, values) = self.get_raw_query();

        let rows = connection.query(
            &*query,
            slice_query_value_iter(values.as_slice()).collect::<Vec<&(dyn ToSql + Sync)>>().as_slice()
        ).await?;

        Ok(rows)
    }
}