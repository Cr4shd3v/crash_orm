//! # Query
//! Crash ORM has a unique query API to provide only suitable functions in query.
//!
//! For example, a nullable column needs the condition is_null and is_not_null.
//! But those function won't appear on not nullable columns.
//!
//! In short, you can't produce type errors in your queries, unless there is a bug in the ORM.
//!
//! ## Base Query
//! Every Query on every entity has some basic functions.
//! In general, all queries are based on the same functions.
//!
//! ### Creating a Query
//! Queries can only be created at an entity.
//!
//! Example:
//!
//! ```rust
//! use crash_orm::prelude::*;
//!
//! # #[derive(Entity, Debug, Schema)]
//! # struct TestEntity {
//! #    id: Option<u32>,
//! #    test: u32,
//! # }
//!
//! let mut query = TestEntity::query();
//! ```
//!
//! A query has the entity as generic type.
//! That means, that all the following parameters must be from this entity.
//!
//! ### Set condition
//! Once you created a query, you might want to add a condition.
//! This will be translated to the WHERE clause in Postgres.
//!
//! Example:
//!
//! ```rust
//! use crash_orm::prelude::*;
//!
//! # #[derive(Entity, Debug, Schema)]
//! # struct TestEntity {
//! #    id: Option<u32>,
//! #    test: u32,
//! # }
//!
//! // WHERE id = 1
//! let mut query = TestEntity::query()
//!     .condition(TestEntityColumn::ID.equals(1));
//! ```
//!
//! A detailed documentation of all available QueryConditions can be found [here](../../crash_orm/src/query_condition.rs).
//!
//! ### Set Order
//! You can also set multiple orders for the query.
//! This will be translated to ORDER BYs.
//!
//! Example:
//! ```rust
//! use crash_orm::prelude::*;
//!
//! # #[derive(Entity, Debug, Schema)]
//! # struct TestEntity {
//! #    id: Option<u32>,
//! #    test: u32,
//! # }
//!
//! let mut query = TestEntity::query()
//!     .order(&TestEntityColumn::ID, OrderDirection::DESC)
//!     .add_order(&TestEntityColumn::TEST, OrderDirection::ASC);
//! ```
//!
//! ### Execute Query
//! When you are done building the query, you can finally execute it.
//!
//! ```rust
//! use crash_orm::prelude::*;
//! # use crash_orm_test::setup_test_connection;
//!
//! # #[derive(Entity, Debug, Schema)]
//! # struct TestEntity {
//! #    id: Option<u32>,
//! # }
//!
//! # tokio_test::block_on(async {
//! # let conn = setup_test_connection().await;
//! # TestEntity::create_table_if_not_exists(&conn).await.unwrap();
//! let results: Vec<TestEntity> = TestEntity::query()
//!     .condition(TestEntityColumn::ID.equals(1))
//!     .fetch(&conn).await.unwrap();
//! # });
//! ```

use std::fmt::Display;
use std::marker::PhantomData;
use std::sync::Arc;

use tokio_postgres::types::ToSql;

use crate::entity::slice_query_value_iter;
use crate::prelude::{BoxedSql, DatabaseConnection, Entity, QueryCondition, UntypedColumn};
use crate::result_mapping::ResultMapping;

/// Direction of the Order
#[derive(Debug)]
pub enum OrderDirection {
    /// ASC
    ASC,
    /// DESC
    DESC,
}

impl Display for OrderDirection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            OrderDirection::ASC => "ASC",
            OrderDirection::DESC => "DESC",
        })
    }
}

/// Struct representing a database query for an entity.
pub struct Query<T: Entity, R: ResultMapping> {
    base_query: BoxedSql,
    condition: Option<QueryCondition<T>>,
    order: Vec<(BoxedSql, OrderDirection)>,
    phantom: PhantomData<R>,
}

impl<T: Entity, R: ResultMapping> Query<T, R> {
    /// Create a new query from a [BoxedSql]
    pub(crate) fn new(base_query: BoxedSql) -> Query<T, R> {
        Self {
            base_query,
            condition: None,
            order: vec![],
            phantom: PhantomData,
        }
    }

    /// Set the condition for this query.
    pub fn condition(mut self, condition: QueryCondition<T>) -> Query<T, R> {
        self.condition = Some(condition);
        self
    }

    /// Add an order to this query.
    pub fn add_order(
        mut self,
        order: &(dyn UntypedColumn<T>),
        order_direction: OrderDirection,
    ) -> Query<T, R> {
        self.order.push((order.get_sql(), order_direction));
        self
    }

    /// Set the order for this query.
    ///
    /// This will OVERRIDE all previous orders.
    pub fn order(
        mut self,
        order: &(dyn UntypedColumn<T>),
        order_direction: OrderDirection,
    ) -> Query<T, R> {
        self.order.clear();
        self.order.push((order.get_sql(), order_direction));
        self
    }

    fn get_raw_query(self) -> (String, Vec<Arc<Box<dyn ToSql + Send + Sync>>>) {
        let (mut query, mut values, mut index) = self.base_query.resolve(1);

        if self.condition.is_some() {
            let (condition_query, condition_values, next_index) =
                self.condition.unwrap().resolve(index);
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

    /// Execute this query and returns the result as a vector of entities.
    pub async fn fetch(self, connection: &impl DatabaseConnection) -> crate::Result<Vec<R>> {
        let (query, values) = self.get_raw_query();

        let rows = connection
            .query_many(
                &*query,
                slice_query_value_iter(values.as_slice())
                    .collect::<Vec<&(dyn ToSql + Sync)>>()
                    .as_slice(),
            )
            .await?;

        Ok(rows.into_iter().map(|r| R::from_row(r)).collect::<Vec<R>>())
    }

    /// Execute this query and returns a single result as an entity
    pub async fn fetch_single(self, connection: &impl DatabaseConnection) -> crate::Result<R> {
        let (query, values) = self.get_raw_query();

        let row = connection
            .query_single(
                &*query,
                slice_query_value_iter(values.as_slice())
                    .collect::<Vec<&(dyn ToSql + Sync)>>()
                    .as_slice(),
            )
            .await?;

        Ok(R::from_row(row))
    }
}
