//! Contains the definition of query conditions.

use std::marker::PhantomData;
use std::sync::Arc;

use tokio_postgres::types::ToSql;

pub use bool_condition::*;
pub use compare_condition::*;
pub use equal_condition::*;
pub use in_condition::*;
pub use like_condition::*;
pub use null_condition::*;

use crate::prelude::{BoxedSql, Entity};

mod null_condition;
mod equal_condition;
mod like_condition;
mod compare_condition;
mod bool_condition;
mod in_condition;

/// Query condition for entity
pub struct QueryCondition<T: Entity> {
    /// [BoxedSql] of this query condition
    pub boxed: BoxedSql,
    phantom: PhantomData<T>,
}

impl<T: Entity> QueryCondition<T> {
    /// Create a new query condition from a [BoxedSql]
    pub fn new(boxed: BoxedSql) -> Self {
        Self {
            boxed,
            phantom: PhantomData,
        }
    }

    pub(crate) fn resolve(
        self,
        index: usize,
    ) -> (String, Vec<Arc<Box<dyn ToSql + Send + Sync>>>, usize) {
        self.boxed.resolve(index)
    }

    /// Build AND condition from self and other
    pub fn and(mut self, other: QueryCondition<T>) -> QueryCondition<T> {
        self.boxed.modify(|v| format!("({v}) AND ({})", other.boxed.sql));
        self.boxed.values.extend(other.boxed.values);
        self
    }

    /// Build OR condition from self and other
    pub fn or(mut self, other: QueryCondition<T>) -> QueryCondition<T> {
        self.boxed.modify(|v| format!("({v}) OR ({})", other.boxed.sql));
        self.boxed.values.extend(other.boxed.values);
        self
    }

    /// Build NOT condition from self
    pub fn not(mut self) -> QueryCondition<T> {
        self.boxed.modify(|v| format!("NOT({v})"));
        self
    }
}
