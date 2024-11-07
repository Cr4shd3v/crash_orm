//! Contains the trait for mapping a [Row] into an object.
//!
//! Also contains the wrapper struct [SingleResult] for easy parsing of a single column result.

use crate::prelude::ColumnType;
use postgres::Row;
use std::ops::{Deref, DerefMut};

/// This trait parses the result of a single row in a query.
///
/// Required for [Entity::select_query] generic type.
pub trait ResultMapping {
    /// Parses Self from a [Row].
    fn from_row(row: Row) -> Self where Self: Sized;
}

impl ResultMapping for Row {
    fn from_row(row: Row) -> Self
    where
        Self: Sized
    {
        row
    }
}

/// Wrapper struct which holds a single element for result mapping from a select query.
///
/// Implements [ResultMapping], supports all types with [ColumnType].
pub struct SingleResult<T: ColumnType> {
    inner: T,
}

impl<T: ColumnType> Deref for SingleResult<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<T: ColumnType> DerefMut for SingleResult<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl<T: ColumnType> ResultMapping for SingleResult<T> {
    fn from_row(row: Row) -> Self
    where
        Self: Sized
    {
        SingleResult {
            inner: row.get::<_, T>(0),
        }
    }
}
