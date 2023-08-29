use tokio_postgres::Row;
use tokio_postgres::types::ToSql;
use crate::{DatabaseConnection, Entity, UntypedColumn, QueryCondition};
use crate::entity::slice_query_value_iter;

#[derive(Debug)]
pub enum OrderDirection {
    ASC,
    DESC,
}

impl ToString for OrderDirection {
    fn to_string(&self) -> String {
        match self {
            OrderDirection::ASC => String::from("ASC"),
            OrderDirection::DESC => String::from("DESC"),
        }
    }
}

macro_rules! base_query_functions {
    ($base:ident) => {
        pub fn new(base_query: String) -> $base<T> {
            Self {
                base_query,
                condition: None,
                order: vec![],
            }
        }

        pub fn condition(mut self, condition: QueryCondition<T>) -> $base<T> {
            self.condition = Some(condition);
            self
        }

        pub fn add_order(mut self, order: &(dyn UntypedColumn<T>), order_direction: OrderDirection) -> $base<T> {
            self.order.push((order.get_sql(), order_direction));
            self
        }

        pub fn order(mut self, order: &(dyn UntypedColumn<T>), order_direction: OrderDirection) -> $base<T> {
            self.order.clear();
            self.order.push((order.get_sql(), order_direction));
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

            if !self.order.is_empty() {
                query.push_str(" ORDER BY ");

                for (order_name, order_dir) in self.order {
                    query.push_str(&*order_name);
                    query.push_str(" ");
                    query.push_str(&*order_dir.to_string());
                    query.push_str(",");
                }

                query = query.strip_suffix(",").unwrap().to_string();
            }

            (query, values)
        }
    };
}

pub struct Query<T: Entity<T>> {
    base_query: String,
    condition: Option<QueryCondition<T>>,
    order: Vec<(String, OrderDirection)>,
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
    order: Vec<(String, OrderDirection)>,
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