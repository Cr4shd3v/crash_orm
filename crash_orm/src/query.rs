use std::sync::Arc;
use tokio_postgres::Row;
use tokio_postgres::types::ToSql;
use crate::{DatabaseConnection, Entity, UntypedColumn, QueryCondition, BoxedColumnValue};
use crate::entity::slice_query_value_iter;

/// Direction of the Order
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
        /// Create a new query from a [BoxedColumnValue]
        pub(crate) fn new(base_query: BoxedColumnValue) -> $base<T> {
            Self {
                base_query,
                condition: None,
                order: vec![],
            }
        }

        /// Set the condition for this query.
        pub fn condition(mut self, condition: QueryCondition<T>) -> $base<T> {
            self.condition = Some(condition);
            self
        }

        /// Add an order to this query.
        pub fn add_order(mut self, order: &(dyn UntypedColumn<T>), order_direction: OrderDirection) -> $base<T> {
            self.order.push((order.get_sql(), order_direction));
            self
        }

        /// Set the order for this query.
        ///
        /// This will OVERRIDE all previous orders.
        pub fn order(mut self, order: &(dyn UntypedColumn<T>), order_direction: OrderDirection) -> $base<T> {
            self.order.clear();
            self.order.push((order.get_sql(), order_direction));
            self
        }

        fn get_raw_query(self) -> (String, Vec<Arc<Box<dyn ToSql+Send+Sync>>>) {
            let (mut query, mut values, mut index) = self.base_query.resolve(1);

            if self.condition.is_some() {
                let (condition_query, condition_values, next_index) = self.condition.unwrap().resolve(index);
                index = next_index;
                values = condition_values;
                query.push_str(" WHERE ");
                query.push_str(&*condition_query);
            }

            if !self.order.is_empty() {
                query.push_str(" ORDER BY ");
                let mut orders = vec![];

                for (order_name, order_dir) in self.order {
                    let (order_query, order_values, _) = order_name.resolve(index);
                    values.extend(order_values);
                    orders.push(format!("{} {}", order_query, order_dir.to_string()));
                }

                query.push_str(&*orders.join(","));
            }

            (query, values)
        }
    };
}

pub struct Query<T: Entity<T>> {
    base_query: BoxedColumnValue,
    condition: Option<QueryCondition<T>>,
    order: Vec<(BoxedColumnValue, OrderDirection)>,
}

impl<T: Entity<T>> Query<T> {
    base_query_functions!(Query);

    /// Execute this query and returns the result as a vector of entities of type [T].
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
    base_query: BoxedColumnValue,
    condition: Option<QueryCondition<T>>,
    order: Vec<(BoxedColumnValue, OrderDirection)>,
}

impl<T: Entity<T>> SelectQuery<T> {
    base_query_functions!(SelectQuery);

    /// Execute this query and returns the result as a vector of [Row].
    ///
    /// This can't be parsed automatically, since the selected columns can be anything you want.
    pub async fn execute(self, connection: &DatabaseConnection) -> crate::Result<Vec<Row>> {
        let (query, values) = self.get_raw_query();

        let rows = connection.query(
            &*query,
            slice_query_value_iter(values.as_slice()).collect::<Vec<&(dyn ToSql + Sync)>>().as_slice()
        ).await?;

        Ok(rows)
    }
}