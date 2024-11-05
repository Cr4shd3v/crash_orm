//! Contains the definition of a column of an entity.
//!
//! These columns will be auto generated by the derive macro, so you shouldn't need to use them manually.

use std::marker::PhantomData;

use tokio_postgres::types::ToSql;

pub use max_column::*;
pub use min_column::*;
pub use sum_column::*;

use crate::prelude::{BoxedSql, Entity};

mod sum_column;
mod min_column;
mod max_column;

/// Struct holding information about a column of an entity.
///
/// These columns will be automatically generated by the entity derive macro.
pub struct EntityColumn<T: ToSql, U: Entity> {
    name: &'static str,
    phantom_1: PhantomData<T>,
    phantom_2: PhantomData<U>,
}

impl<T: ToSql, U: Entity> EntityColumn<T, U> {
    /// Creates an [EntityColumn] for a column name.
    ///
    /// INTERNAL USE ONLY!
    #[doc(hidden)]
    pub const fn new(name: &'static str) -> EntityColumn<T, U> {
        Self {
            name,
            phantom_1: PhantomData,
            phantom_2: PhantomData,
        }
    }

    /// Convert [EntityColumn] into a [BoxedSql]
    pub(crate) fn get_sql(&self) -> BoxedSql {
        BoxedSql::new(self.name.to_string(), vec![])
    }
}
