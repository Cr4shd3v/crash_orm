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
//! #    id: u32,
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
//! #    id: u32,
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
//! #    id: u32,
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
//! #    id: u32,
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
use crate::prelude::{BoxedSql, ColumnType, DatabaseConnection, Entity, EntityColumn, QueryCondition, UntypedColumn};
use crate::result_mapping::ResultMapping;

/// Marks a query as a SELECT query.
pub struct SelectQueryType;

/// Marks a query as a DELETE query.
pub struct DeleteQueryType;

/// Direction of the Order
#[derive(Debug, Clone)]
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
pub struct Query<T: Entity, R: ResultMapping, QT> {
    base_query: BoxedSql,
    condition: Option<QueryCondition<T>>,
    group_by: Vec<BoxedSql>,
    order: Vec<(BoxedSql, OrderDirection)>,
    phantom: PhantomData<(R, QT)>,
}

impl<T: Entity, R: ResultMapping, QT> Query<T, R, QT> {
    /// Create a new query from a [BoxedSql]
    pub fn new(base_query: BoxedSql) -> Query<T, R, QT> {
        Self {
            base_query,
            condition: None,
            group_by: vec![],
            order: vec![],
            phantom: PhantomData,
        }
    }

    /// Set the condition for this query.
    pub fn condition(mut self, condition: QueryCondition<T>) -> Query<T, R, QT> {
        self.condition = Some(condition);
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
        
        if !self.group_by.is_empty() {
            query.push_str(" GROUP BY ");
            
            let mut grouped_by = vec![];

            for x in self.group_by {
                grouped_by.push(x.sql);
            }
            
            query.push_str(&*grouped_by.join(","));
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
}

impl<T: Entity, R: ResultMapping> Query<T, R, SelectQueryType> {
    /// Add an order to this query.
    pub fn add_order(
        mut self,
        order: &(dyn UntypedColumn<T>),
        order_direction: OrderDirection,
    ) -> Query<T, R, SelectQueryType> {
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
    ) -> Query<T, R, SelectQueryType> {
        self.order = vec![(order.get_sql(), order_direction)];
        self
    }

    /// Add a grouping to this query
    pub fn add_group_by<U: ColumnType>(
        mut self,
        group_by: &EntityColumn<U ,T>,
    ) -> Query<T, R, SelectQueryType> {
        self.group_by.push(group_by.get_sql());
        self
    }

    /// Set the grouping for this query.
    ///
    /// This will OVERRIDE all previous grouping.
    pub fn group_by<U: ColumnType>(
        mut self,
        group_by: &EntityColumn<U ,T>,
    ) -> Query<T, R, SelectQueryType> {
        self.group_by = vec![group_by.get_sql()];
        self
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

        Ok(rows.into_iter().map(|r| R::from_row(r)).filter(|r| r.is_some()).map(|r| r.unwrap()).collect::<Vec<R>>())
    }

    /// Execute this query and returns a single result as an entity
    pub async fn fetch_single(self, connection: &impl DatabaseConnection) -> crate::Result<Option<R>> {
        let (query, values) = self.get_raw_query();

        let row = connection
            .query_single(
                &*query,
                slice_query_value_iter(values.as_slice())
                    .collect::<Vec<&(dyn ToSql + Sync)>>()
                    .as_slice(),
            )
            .await?;
        
        if let Some(row) = row {
            Ok(R::from_row(row))
        } else {    
            Ok(None)
        }
    }
}

impl<T: Entity, R: ResultMapping> Query<T, R, DeleteQueryType> {
    /// Execute this query without a result
    pub async fn execute(self, connection: &impl DatabaseConnection) -> crate::Result<()> {
        let (query, values) = self.get_raw_query();

        connection
            .execute_query(
                &*query,
                slice_query_value_iter(values.as_slice())
                    .collect::<Vec<&(dyn ToSql + Sync)>>()
                    .as_slice(),
            )
            .await?;

        Ok(())
    }
}
