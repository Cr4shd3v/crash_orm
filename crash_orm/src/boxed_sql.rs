//! Contains the [BoxedSql] struct and the [IntoSql] trait.
//!
//! Utility for building a full query.

use std::sync::Arc;

use postgres::types::ToSql;

use crate::column_value::UntypedColumnValue;
use crate::prelude::ColumnType;

/// Struct containing a part of a query with raw sql and values prepared for tokio-postgres.
#[derive(Clone, Debug)]
pub struct BoxedSql {
    /// Raw SQL string representing this part of a query.
    ///
    /// NOTE: There will be placeholders (_$i) which will be resolved once the query is complete.
    pub sql: String,
    /// Contains all values as a `Arc<Box<>>`
    pub values: Vec<Arc<Box<dyn ToSql + Sync + Send + 'static>>>,
}

impl BoxedSql {
    /// Creates a new instance
    pub fn new(
        sql: String,
        value: Vec<Arc<Box<dyn ToSql + Sync + Send + 'static>>>,
    ) -> Self {
        Self { sql, values: value }
    }

    /// Resolves this value into it's parts with inserted IDs
    pub fn resolve(
        mut self,
        mut index: usize,
    ) -> (String, Vec<Arc<Box<dyn ToSql + Sync + Send>>>, usize) {
        while self.sql.contains("_$i") {
            self.sql = self.sql.replacen("_$i", &*format!("${index}"), 1);
            index += 1;
        }

        (self.sql, self.values, index)
    }

    /// Modify the raw sql string.
    pub fn modify<F: FnOnce(&String) -> String>(&mut self, f: F) {
        self.sql = f(&self.sql);
    }
}

/// Trait for converting any type that implements [ColumnType] and [UntypedColumnValue] into a [BoxedSql].
#[allow(clippy::wrong_self_convention)]
pub trait IntoSql<T> {
    /// Convert self into a [BoxedSql]
    fn into_boxed_sql(&self) -> BoxedSql;
}

impl<T: ColumnType + UntypedColumnValue> IntoSql<T> for T {
    fn into_boxed_sql(&self) -> BoxedSql {
        self.get_sql()
    }
}

impl<T: ColumnType + UntypedColumnValue> IntoSql<T> for &T {
    fn into_boxed_sql(&self) -> BoxedSql {
        self.get_sql()
    }
}

impl<'a> IntoSql<String> for &'a str {
    fn into_boxed_sql(&self) -> BoxedSql {
        self.to_string().into_boxed_sql()
    }
}