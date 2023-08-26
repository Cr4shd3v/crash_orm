use std::marker::PhantomData;
use async_trait::async_trait;
use tokio_postgres::types::ToSql;
use crate::Entity;

#[async_trait]
pub trait QueryEntity<T: Entity + Send + 'static>: Entity {
    async fn query(condition: QueryCondition<T>) -> crate::Result<Vec<T>> {
        Ok(vec![])
    }
}

pub enum QueryCondition<T: Entity + Send> {
    Equals(String, Box<dyn ToSql + Send>),
    NotEquals(String, Box<dyn ToSql + Send>),
    And(Box<QueryCondition<T>>, Box<QueryCondition<T>>),
    Or(Box<QueryCondition<T>>, Box<QueryCondition<T>>),
    #[allow(non_camel_case_types)]__(PhantomData<T>),
}

impl<T: Entity + Send> QueryCondition<T> {
    pub fn and(self, other: QueryCondition<T>) -> QueryCondition<T> {
        QueryCondition::And(Box::new(self), Box::new(other))
    }

    pub fn or(self, other: QueryCondition<T>) -> QueryCondition<T> {
        QueryCondition::Or(Box::new(self), Box::new(other))
    }
}

pub struct QueryColumn<T: ToSql, U: Entity> {
    name: &'static str,
    phantom_1: PhantomData<T>,
    phantom_2: PhantomData<U>,
}

impl<T: ToSql, U: Entity> QueryColumn<T, U> {
    pub const fn new(name: &'static str) -> QueryColumn<T, U> {
        Self {
            name,
            phantom_1: PhantomData,
            phantom_2: PhantomData,
        }
    }
}

pub trait EqualQueryColumn<T: ToSql, U: Entity + Send> {
    fn equals(&self, other: T) -> QueryCondition<U>;

    fn not_equals(&self, other: T) -> QueryCondition<U>;
}

macro_rules! impl_equal_query_column {
    ($column_type:ty) => {
        impl<T: Entity + Send> EqualQueryColumn<$column_type, T> for QueryColumn<$column_type, T> {
            fn equals(&self, other: $column_type) -> QueryCondition<T> {
                QueryCondition::Equals(self.name.to_string(), Box::new(other))
            }

            fn not_equals(&self, other: $column_type) -> QueryCondition<T> {
                QueryCondition::NotEquals(self.name.to_string(), Box::new(other))
            }
        }
    };
}

impl_equal_query_column!(bool);
impl_equal_query_column!(i8);
impl_equal_query_column!(i16);
impl_equal_query_column!(i32);
impl_equal_query_column!(i64);
impl_equal_query_column!(u32);
impl_equal_query_column!(f32);
impl_equal_query_column!(f64);
impl_equal_query_column!(String);