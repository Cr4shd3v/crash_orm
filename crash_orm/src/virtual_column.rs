mod length_column;

use std::marker::PhantomData;
use tokio_postgres::types::ToSql;
pub use length_column::*;
use crate::Entity;

pub struct VirtualColumn<T: ToSql, U: Entity<U> + Send + 'static> {
    name: String,
    phantom_1: PhantomData<T>,
    phantom_2: PhantomData<U>,
}

impl<T: ToSql, U: Entity<U> + Send + 'static> VirtualColumn<T, U> {
    pub fn new(name: String) -> VirtualColumn<T, U> {
        VirtualColumn {
            name,
            phantom_1: PhantomData,
            phantom_2: PhantomData,
        }
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }
}