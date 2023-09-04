mod length_column;
pub use length_column::*;

mod string_case_column;
pub use string_case_column::*;

mod string_reverse_column;
pub use string_reverse_column::*;

mod round_column;
pub use round_column::*;

mod sqrt_column;
pub use sqrt_column::*;

use std::marker::PhantomData;
use tokio_postgres::types::ToSql;
use crate::{BoxedColumnValue, Entity};

pub struct VirtualColumn<T: ToSql, U: Entity<U>> {
    sql: BoxedColumnValue,
    phantom_1: PhantomData<T>,
    phantom_2: PhantomData<U>,
}

impl<T: ToSql, U: Entity<U>> VirtualColumn<T, U> {
    pub(crate) fn new(sql: BoxedColumnValue) -> VirtualColumn<T, U> {
        VirtualColumn {
            sql,
            phantom_1: PhantomData,
            phantom_2: PhantomData,
        }
    }

    pub fn get_sql(&self) -> BoxedColumnValue {
        self.sql.clone()
    }
}