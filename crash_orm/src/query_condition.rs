use std::marker::PhantomData;
use tokio_postgres::types::ToSql;
use crate::Entity;

mod null_condition;
pub use null_condition::*;

mod equal_condition;
pub use equal_condition::*;

mod like_condition;
pub use like_condition::*;

mod compare_condition;
pub use compare_condition::*;

mod bool_condition;
pub use bool_condition::*;

mod in_condition;
pub use in_condition::*;

pub enum QueryCondition<T: Entity<T>> {
    Equals(String, String),
    NotEquals(String, String),
    And(Box<QueryCondition<T>>, Box<QueryCondition<T>>),
    Or(Box<QueryCondition<T>>, Box<QueryCondition<T>>),
    IsNull(String),
    IsNotNull(String),
    Not(Box<QueryCondition<T>>),
    Like(String, String),
    NotLike(String, String),
    GreaterThan(String, String),
    GreaterEqual(String, String),
    LessThan(String, String),
    LessEqual(String, String),
    Between(String, String, String),
    NotBetween(String, String, String),
    IsTrue(String),
    IsFalse(String),
    In(String, Vec<String>),
    NotIn(String, Vec<String>),
    #[allow(non_camel_case_types)]__(PhantomData<T>),
}

impl<T: Entity<T>> QueryCondition<T> {
    pub(crate) fn resolve(self, index: usize) -> (String, Vec<Box<dyn ToSql + Send + Sync>>, usize) {
        match self {
            QueryCondition::Equals(name, value) => {
                (format!("{} = {}", name, value), vec![], index)
            },
            QueryCondition::NotEquals(name, value) => {
                (format!("{} <> {}", name, value), vec![], index)
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
            QueryCondition::Like(name, like) => {
                (format!("{} LIKE ${}", name, index), vec![Box::new(like)], index + 1)
            }
            QueryCondition::NotLike(name, like) => {
                (format!("{} NOT LIKE ${}", name, index), vec![Box::new(like)], index + 1)
            }
            QueryCondition::GreaterThan(name, value) => {
                (format!("{} > {}", name, value), vec![], index)
            }
            QueryCondition::GreaterEqual(name, value) => {
                (format!("{} >= {}", name, value), vec![], index)
            }
            QueryCondition::LessThan(name, value) => {
                (format!("{} < {}", name, value), vec![], index)
            }
            QueryCondition::LessEqual(name, value) => {
                (format!("{} <= {}", name, value), vec![], index)
            }
            QueryCondition::Between(name, from, to) => {
                (format!("{} BETWEEN {} AND {}", name, from, to), vec![], index)
            }
            QueryCondition::NotBetween(name, from, to) => {
                (format!("{} NOT BETWEEN {} AND {}", name, from, to), vec![], index)
            }
            QueryCondition::IsTrue(name) => {
                (format!("{} IS TRUE", name), vec![], index)
            }
            QueryCondition::IsFalse(name) => {
                (format!("{} IS FALSE", name), vec![], index)
            }
            QueryCondition::In(name, values) => {
                let mut format_string = String::new();
                for value in values {
                    format_string.push_str(&*format!("{},", value))
                }

                (format!("{} IN ({})", name, format_string.strip_suffix(",").unwrap()), vec![], index)
            }
            QueryCondition::NotIn(name, values) => {
                let mut format_string = String::new();
                for value in values {
                    format_string.push_str(&*format!("{},", value))
                }

                (format!("{} NOT IN ({})", name, format_string.strip_suffix(",").unwrap()), vec![], index)
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