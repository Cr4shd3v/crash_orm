use std::marker::PhantomData;
use tokio_postgres::types::ToSql;
use crate::Entity;

pub struct EntityColumn<T: ToSql, U: Entity<U> + Send + 'static> {
    pub(crate) name: &'static str,
    phantom_1: PhantomData<T>,
    phantom_2: PhantomData<U>,
}

impl<T: ToSql, U: Entity<U> + Send + 'static> EntityColumn<T, U> {
    pub const fn new(name: &'static str) -> EntityColumn<T, U> {
        Self {
            name,
            phantom_1: PhantomData,
            phantom_2: PhantomData,
        }
    }

    pub fn get_name(&self) -> &str {
        self.name
    }
}