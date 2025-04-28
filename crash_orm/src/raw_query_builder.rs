//! This module contains the [RawQueryBuilder] which can build select queries with raw SQL.
//! 
//! Be careful: There aren't any safety measures to make sure that the query is valid!

use std::sync::Arc;
use postgres::types::ToSql;
use crate::prelude::{slice_query_value_iter, BoxedSql, DatabaseConnection, Entity, ResultMapping};

/// The [RawQueryBuilder] can build select queries with raw SQL.
///
/// Be careful: There aren't any safety measures to make sure that the query is valid!
#[derive(Clone, Debug, Default)]
pub struct RawQueryBuilder {
    selects: Vec<String>,
    from: Vec<String>,
    where_statement: Vec<BoxedSql>,
    joins: Vec<String>,
    order_by: Vec<String>,
    group_by: Vec<String>,
}

impl RawQueryBuilder {
    /// Adds a select statement to the query
    pub fn add_select(&mut self, select: impl Into<String>) -> &mut Self {
        self.selects.push(select.into());
        self
    }

    /// Adds a from statement to the query
    pub fn add_from(&mut self, from: impl Into<String>, alias: impl Into<String>) -> &mut Self {
        self.from.push(format!("{} {}", from.into(), alias.into()));
        self
    }
    
    /// Shortcut for adding a from statement from an entity
    pub fn add_from_entity<T: Entity>(&mut self, alias: impl Into<String>) -> &mut Self {
        self.add_from(T::TABLE_NAME, alias)
    }

    /// Adds a where statement to the query
    pub fn and_where(&mut self, sql: impl Into<String>, values: Vec<Arc<Box<dyn ToSql + Sync + Send + 'static>>>) -> &mut Self {
        self.where_statement.push(BoxedSql::new(sql.into(), values));
        self
    }

    /// Adds a left join to the query
    pub fn left_join(&mut self, sql: impl Into<String>) -> &mut Self {
        self.joins.push(format!(" LEFT JOIN {}", sql.into()));
        self
    }

    /// Adds a join to the query
    pub fn join(&mut self, sql: impl Into<String>) -> &mut Self {
        self.joins.push(format!(" JOIN {}", sql.into()));
        self
    }

    /// Adds an inner join to the query
    pub fn inner_join(&mut self, sql: impl Into<String>) -> &mut Self {
        self.joins.push(format!(" INNER JOIN {}", sql.into()));
        self
    }

    /// Adds an order by to the query
    pub fn add_order_by(&mut self, order_by: impl Into<String>) -> &mut Self {
        self.order_by.push(order_by.into());
        self
    }

    /// Adds a group by to the query
    pub fn add_group_by(&mut self, group_by: impl Into<String>) -> &mut Self {
        self.group_by.push(group_by.into());
        self
    }
    
    /// Builds the raw SQL query from this [RawQueryBuilder].
    pub fn build(self) -> (String, Vec<Arc<Box<dyn ToSql + Sync + Send + 'static>>>) {
        let mut query = "SELECT ".to_string();
        let mut values = vec![];
        let mut index = 1;

        query.push_str(self.selects.join(",").as_str());
        query.push_str(" FROM ");
        query.push_str(self.from.join(",").as_str());

        if !self.where_statement.is_empty() {
            query.push_str(" WHERE ");
            for sql in self.where_statement.into_iter() {
                let (sql, where_values, new_index) = sql.resolve(index);
                query.push_str(&*sql);
                values.extend(where_values);
                index = new_index;
            }
        }
        
        if !self.joins.is_empty() {
            query.push_str(&*self.joins.join(","));
        }
        
        if !self.order_by.is_empty() {
            query.push_str(" ORDER BY ");
            query.push_str(&*self.order_by.join(","));
        }
        
        if !self.order_by.is_empty() {
            query.push_str(" GROUP BY ");
            query.push_str(&*self.group_by.join(","));
        }

        (query, values)
    }
    
    /// Build the query and execute it with multiple results.
    pub async fn query_many<R: ResultMapping>(self, conn: &impl DatabaseConnection) -> crate::Result<Vec<R>> {
        let (query, values) = self.build();
        
        Ok(conn.query_many(&*query, slice_query_value_iter(values.as_slice())
            .collect::<Vec<&(dyn ToSql + Sync)>>()
            .as_slice()).await?.into_iter().map(|r| R::from_row(r).unwrap()).collect())
    }
}
