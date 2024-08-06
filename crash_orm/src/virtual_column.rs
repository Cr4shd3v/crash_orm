//! See [VirtualColumn].

use std::marker::PhantomData;

use tokio_postgres::types::ToSql;

pub use round_column::*;
pub use sqrt_column::*;
pub use string_column::*;
pub use text_cast_column::*;

use crate::prelude::{BoxedColumnValue, Entity, PrimaryKey};

mod string_column;
mod round_column;
mod sqrt_column;
mod text_cast_column;

/// Struct holding information about a non-existing column. This can be for example SQRT(number).
///
/// These columns can be automatically generated by calling the corresponding functions, you cannot create them yourself!
///
/// This also means, that you cannot add your own virtual columns.
/// If you need more virtual columns, please [open an issue at the repository](https://github.com/Cr4shd3v/crash_orm/issues/new/choose).
pub struct VirtualColumn<T: ToSql, U: Entity<U, P>, P: PrimaryKey> {
    sql: BoxedColumnValue,
    phantom_1: PhantomData<T>,
    phantom_2: PhantomData<U>,
    phantom_3: PhantomData<P>,
}

impl<T: ToSql, U: Entity<U, P>, P: PrimaryKey> VirtualColumn<T, U, P> {
    /// Creates a virtual column with a [BoxedColumnValue]
    pub(crate) fn new(sql: BoxedColumnValue) -> VirtualColumn<T, U, P> {
        VirtualColumn {
            sql,
            phantom_1: PhantomData,
            phantom_2: PhantomData,
            phantom_3: PhantomData,
        }
    }

    pub(crate) fn get_sql(&self) -> BoxedColumnValue {
        self.sql.clone()
    }

    /// Constant Column Pi
    pub fn pi() -> VirtualColumn<f64, U, P> {
        VirtualColumn::new(BoxedColumnValue::new(String::from("PI()"), vec![]))
    }

    /// Generates a random value in the range 0.0 <= x < 1.0
    pub fn random() -> VirtualColumn<f64, U, P> {
        VirtualColumn::new(BoxedColumnValue::new(String::from("RANDOM()"), vec![]))
    }
}
