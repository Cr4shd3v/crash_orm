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
    Equals(String, Box<dyn ToSql + Sync + Send>),
    NotEquals(String, Box<dyn ToSql + Sync + Send>),
    And(Box<QueryCondition<T>>, Box<QueryCondition<T>>),
    Or(Box<QueryCondition<T>>, Box<QueryCondition<T>>),
    IsNull(String),
    IsNotNull(String),
    Not(Box<QueryCondition<T>>),
    Like(String, String),
    NotLike(String, String),
    GreaterThan(String, Box<dyn ToSql + Sync + Send>),
    GreaterEqual(String, Box<dyn ToSql + Sync + Send>),
    LessThan(String, Box<dyn ToSql + Sync + Send>),
    LessEqual(String, Box<dyn ToSql + Sync + Send>),
    Between(String, Box<dyn ToSql + Sync + Send>, Box<dyn ToSql + Sync + Send>),
    NotBetween(String, Box<dyn ToSql + Sync + Send>, Box<dyn ToSql + Sync + Send>),
    IsTrue(String),
    IsFalse(String),
    In(String, Vec<Box<dyn ToSql + Sync + Send>>),
    NotIn(String, Vec<Box<dyn ToSql + Sync + Send>>),
    #[allow(non_camel_case_types)]__(PhantomData<T>),
}

impl<T: Entity<T>> QueryCondition<T> {
    pub(crate) fn resolve(self, index: usize) -> (String, Vec<Box<dyn ToSql + Send + Sync>>, usize) {
        match self {
            QueryCondition::Equals(name, value) => {
                (format!("{} = ${}", name, index), vec![value], index + 1)
            },
            QueryCondition::NotEquals(name, value) => {
                (format!("{} <> ${}", name, index), vec![value], index + 1)
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
                (format!("{} > ${}", name, index), vec![value], index + 1)
            }
            QueryCondition::GreaterEqual(name, value) => {
                (format!("{} >= ${}", name, index), vec![value], index + 1)
            }
            QueryCondition::LessThan(name, value) => {
                (format!("{} < ${}", name, index), vec![value], index + 1)
            }
            QueryCondition::LessEqual(name, value) => {
                (format!("{} <= ${}", name, index), vec![value], index + 1)
            }
            QueryCondition::Between(name, from, to) => {
                (format!("{} BETWEEN ${} AND ${}", name, index, index + 1), vec![from, to], index + 2)
            }
            QueryCondition::NotBetween(name, from, to) => {
                (format!("{} NOT BETWEEN ${} AND ${}", name, index, index + 1), vec![from, to], index + 2)
            }
            QueryCondition::IsTrue(name) => {
                (format!("{} IS TRUE", name), vec![], index)
            }
            QueryCondition::IsFalse(name) => {
                (format!("{} IS FALSE", name), vec![], index)
            }
            QueryCondition::In(name, values) => {
                let mut format_string = String::new();
                let length = values.len();
                for i in 0..length {
                    format_string.push_str(&*format!("${},", index + i))
                }

                (format!("{} IN ({})", name, format_string.strip_suffix(",").unwrap()), values, index + length)
            }
            QueryCondition::NotIn(name, values) => {
                let mut format_string = String::new();
                let length = values.len();
                for i in 0..length {
                    format_string.push_str(&*format!("${},", index + i))
                }

                (format!("{} NOT IN ({})", name, format_string.strip_suffix(",").unwrap()), values, index + length)
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