use std::marker::PhantomData;
use tokio_postgres::types::ToSql;
use crate::Entity;

mod null_condition;
pub use null_condition::*;

mod equal_condition;
pub use equal_condition::*;

pub enum QueryCondition<T: Entity<T> + Send + 'static> {
    Equals(String, Box<dyn ToSql + Sync + Send>),
    NotEquals(String, Box<dyn ToSql + Sync + Send>),
    And(Box<QueryCondition<T>>, Box<QueryCondition<T>>),
    Or(Box<QueryCondition<T>>, Box<QueryCondition<T>>),
    IsNull(String),
    IsNotNull(String),
    Not(Box<QueryCondition<T>>),
    #[allow(non_camel_case_types)]__(PhantomData<T>),
}

impl<T: Entity<T> + Send + 'static> QueryCondition<T> {
    pub(crate) fn resolve(self, index: usize) -> (String, Vec<Box<dyn ToSql + Send + Sync>>, usize) {
        match self {
            QueryCondition::Equals(name, value) => {
                (format!("{} = ${}", name, index), vec![value], index + 1)
            },
            QueryCondition::NotEquals(name, value) => {
                (format!("{} != ${}", name, index), vec![value], index + 1)
            },
            QueryCondition::And(first, second) => {
                let (first_query, mut first_values, index) = first.resolve(index);
                let (second_query, second_values, index) = second.resolve(index);

                first_values.extend(second_values);

                (format!("({}) AND ({})", first_query, second_query), first_values, index)
            }
            QueryCondition::Or(first, second) => {
                let (first_query, mut first_values, index) = first.resolve(index);
                let (second_query, second_values, index) = second.resolve(index);

                first_values.extend(second_values);

                (format!("({}) OR ({})", first_query, second_query), first_values, index)
            }
            QueryCondition::__(_) => {
                panic!("Invalid Condition (PhantomData)");
            }
            QueryCondition::IsNull(name) => {
                (format!("{} IS NULL", name), vec![], index)
            }
            QueryCondition::IsNotNull(name) => {
                (format!("{} IS NOT NULL", name), vec![], index)
            }
            QueryCondition::Not(other) => {
                let (query, values, index) = other.resolve(index);

                (format!("NOT ({})", query), values, index)
            }
        }
    }

    pub fn and(self, other: QueryCondition<T>) -> QueryCondition<T> {
        QueryCondition::And(Box::new(self), Box::new(other))
    }

    pub fn or(self, other: QueryCondition<T>) -> QueryCondition<T> {
        QueryCondition::Or(Box::new(self), Box::new(other))
    }

    pub fn not(self) -> QueryCondition<T> {
        QueryCondition::Not(Box::new(self))
    }
}