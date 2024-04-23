use std::marker::PhantomData;

use tokio_postgres::types::ToSql;

pub use round_column::*;
pub use sqrt_column::*;
pub use string_column::*;
pub use text_cast_column::*;

use crate::{BoxedColumnValue, Entity, PrimaryKey};

mod string_column;
mod round_column;
mod sqrt_column;
mod text_cast_column;

pub struct VirtualColumn<T: ToSql, U: Entity<U, P>, P: PrimaryKey> {
    sql: BoxedColumnValue,
    phantom_1: PhantomData<T>,
    phantom_2: PhantomData<U>,
    phantom_3: PhantomData<P>,
}

impl<T: ToSql, U: Entity<U, P>, P: PrimaryKey> VirtualColumn<T, U, P> {
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

    /// Constant Column Pi ([`f64::PI`])
    pub fn pi() -> VirtualColumn<f64, U, P> {
        VirtualColumn::new(BoxedColumnValue::new(String::from("PI()"), vec![]))
    }

    /// Generates a random value in the range 0.0 <= x < 1.0
    pub fn random() -> VirtualColumn<f64, U, P> {
        VirtualColumn::new(BoxedColumnValue::new(String::from("RANDOM()"), vec![]))
    }
}
