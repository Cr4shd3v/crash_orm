use crate::{BoxedColumnValue, Entity};
use std::marker::PhantomData;
use std::sync::Arc;
use tokio_postgres::types::ToSql;

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

/// Query condition for entity [T]
pub enum QueryCondition<T: Entity<T>> {
    /// SQL: v1 = v2
    Equals(BoxedColumnValue, BoxedColumnValue),
    /// SQL: v1 <> v2
    NotEquals(BoxedColumnValue, BoxedColumnValue),
    /// SQL: (c1) AND (c2)
    And(Box<QueryCondition<T>>, Box<QueryCondition<T>>),
    /// SQL: (c1) OR (c2)
    Or(Box<QueryCondition<T>>, Box<QueryCondition<T>>),
    /// SQL: v1 IS NULL
    IsNull(BoxedColumnValue),
    /// SQL: v1 IS NOT NULL
    IsNotNull(BoxedColumnValue),
    /// SQL: NOT (c1)
    Not(Box<QueryCondition<T>>),
    /// SQL: v1 LIKE v2
    Like(BoxedColumnValue, BoxedColumnValue),
    /// SQL: v1 NOT LIKE v2
    NotLike(BoxedColumnValue, BoxedColumnValue),
    /// SQL: v1 > v2
    GreaterThan(BoxedColumnValue, BoxedColumnValue),
    /// SQL: v1 >= v2
    GreaterEqual(BoxedColumnValue, BoxedColumnValue),
    /// SQL: v1 < v2
    LessThan(BoxedColumnValue, BoxedColumnValue),
    /// SQL: v1 <= v2
    LessEqual(BoxedColumnValue, BoxedColumnValue),
    /// SQL: v1 BETWEEN v2 AND v3
    Between(BoxedColumnValue, BoxedColumnValue, BoxedColumnValue),
    /// SQL: v1 NOT BETWEEN v2 AND v3
    NotBetween(BoxedColumnValue, BoxedColumnValue, BoxedColumnValue),
    /// SQL: v1 IS TRUE
    IsTrue(BoxedColumnValue),
    /// SQL: v1 IS FALSE
    IsFalse(BoxedColumnValue),
    /// SQL: v1 IN (v2)
    In(BoxedColumnValue, Vec<BoxedColumnValue>),
    /// SQL: v1 NOT IN (v2)
    NotIn(BoxedColumnValue, Vec<BoxedColumnValue>),
    /// INTERNAL
    #[allow(non_camel_case_types)]
    __(PhantomData<T>),
}

impl<T: Entity<T>> QueryCondition<T> {
    pub(crate) fn resolve(
        self,
        index: usize,
    ) -> (String, Vec<Arc<Box<dyn ToSql + Send + Sync>>>, usize) {
        match self {
            QueryCondition::Equals(name, value) => {
                let (name_query, mut name_values, index) = name.resolve(index);
                let (value_query, value_values, index) = value.resolve(index);
                name_values.extend(value_values);

                (
                    format!("{} = {}", name_query, value_query),
                    name_values,
                    index,
                )
            }
            QueryCondition::NotEquals(name, value) => {
                let (name_query, mut name_values, index) = name.resolve(index);
                let (value_query, value_values, index) = value.resolve(index);
                name_values.extend(value_values);

                (
                    format!("{} <> {}", name_query, value_query),
                    name_values,
                    index,
                )
            }
            QueryCondition::And(first, second) => {
                let (first_query, mut first_values, index) = first.resolve(index);
                let (second_query, second_values, index) = second.resolve(index);

                first_values.extend(second_values);

                (
                    format!("({}) AND ({})", first_query, second_query),
                    first_values,
                    index,
                )
            }
            QueryCondition::Or(first, second) => {
                let (first_query, mut first_values, index) = first.resolve(index);
                let (second_query, second_values, index) = second.resolve(index);

                first_values.extend(second_values);

                (
                    format!("({}) OR ({})", first_query, second_query),
                    first_values,
                    index,
                )
            }
            QueryCondition::__(_) => {
                panic!("Invalid Condition (PhantomData)");
            }
            QueryCondition::IsNull(name) => {
                let (name_query, name_values, index) = name.resolve(index);

                (format!("{} IS NULL", name_query), name_values, index)
            }
            QueryCondition::IsNotNull(name) => {
                let (name_query, name_values, index) = name.resolve(index);

                (format!("{} IS NOT NULL", name_query), name_values, index)
            }
            QueryCondition::Not(other) => {
                let (query, values, index) = other.resolve(index);

                (format!("NOT ({})", query), values, index)
            }
            QueryCondition::Like(name, like) => {
                let (name_query, mut name_values, index) = name.resolve(index);
                let (like_query, like_values, index) = like.resolve(index);
                name_values.extend(like_values);

                (
                    format!("{} LIKE {}", name_query, like_query),
                    name_values,
                    index,
                )
            }
            QueryCondition::NotLike(name, like) => {
                let (name_query, mut name_values, index) = name.resolve(index);
                let (like_query, like_values, index) = like.resolve(index);
                name_values.extend(like_values);

                (
                    format!("{} NOT LIKE {}", name_query, like_query),
                    name_values,
                    index,
                )
            }
            QueryCondition::GreaterThan(name, value) => {
                let (name_query, mut name_values, index) = name.resolve(index);
                let (value_query, value_values, index) = value.resolve(index);
                name_values.extend(value_values);

                (
                    format!("{} > {}", name_query, value_query),
                    name_values,
                    index,
                )
            }
            QueryCondition::GreaterEqual(name, value) => {
                let (name_query, mut name_values, index) = name.resolve(index);
                let (value_query, value_values, index) = value.resolve(index);
                name_values.extend(value_values);

                (
                    format!("{} >= {}", name_query, value_query),
                    name_values,
                    index,
                )
            }
            QueryCondition::LessThan(name, value) => {
                let (name_query, mut name_values, index) = name.resolve(index);
                let (value_query, value_values, index) = value.resolve(index);
                name_values.extend(value_values);

                (
                    format!("{} < {}", name_query, value_query),
                    name_values,
                    index,
                )
            }
            QueryCondition::LessEqual(name, value) => {
                let (name_query, mut name_values, index) = name.resolve(index);
                let (value_query, value_values, index) = value.resolve(index);
                name_values.extend(value_values);

                (
                    format!("{} <= {}", name_query, value_query),
                    name_values,
                    index,
                )
            }
            QueryCondition::Between(name, from, to) => {
                let (name_query, mut name_values, index) = name.resolve(index);
                let (from_query, from_values, index) = from.resolve(index);
                let (to_query, to_values, index) = to.resolve(index);
                name_values.extend(from_values);
                name_values.extend(to_values);

                (
                    format!("{} BETWEEN {} AND {}", name_query, from_query, to_query),
                    name_values,
                    index,
                )
            }
            QueryCondition::NotBetween(name, from, to) => {
                let (name_query, mut name_values, index) = name.resolve(index);
                let (from_query, from_values, index) = from.resolve(index);
                let (to_query, to_values, index) = to.resolve(index);
                name_values.extend(from_values);
                name_values.extend(to_values);

                (
                    format!("{} NOT BETWEEN {} AND {}", name_query, from_query, to_query),
                    name_values,
                    index,
                )
            }
            QueryCondition::IsTrue(name) => {
                let (name_query, name_values, index) = name.resolve(index);

                (format!("{} IS TRUE", name_query), name_values, index)
            }
            QueryCondition::IsFalse(name) => {
                let (name_query, name_values, index) = name.resolve(index);

                (format!("{} IS FALSE", name_query), name_values, index)
            }
            QueryCondition::In(name, values) => {
                let (name_query, mut name_values, mut index) = name.resolve(index);

                let mut list = vec![];
                for value in values {
                    let (value_query, value_values, next_index) = value.resolve(index);
                    index = next_index;
                    list.push(value_query);
                    name_values.extend(value_values);
                }

                (
                    format!("{} IN ({})", name_query, list.join(",")),
                    name_values,
                    index,
                )
            }
            QueryCondition::NotIn(name, values) => {
                let (name_query, mut name_values, mut index) = name.resolve(index);

                let mut list = vec![];
                for value in values {
                    let (value_query, value_values, next_index) = value.resolve(index);
                    index = next_index;
                    list.push(value_query);
                    name_values.extend(value_values);
                }

                (
                    format!("{} NOT IN ({})", name_query, list.join(",")),
                    name_values,
                    index,
                )
            }
        }
    }

    /// Build [QueryCondition::And] from self and other
    pub fn and(self, other: QueryCondition<T>) -> QueryCondition<T> {
        QueryCondition::And(Box::new(self), Box::new(other))
    }

    /// Build [QueryCondition::Or] from self and other
    pub fn or(self, other: QueryCondition<T>) -> QueryCondition<T> {
        QueryCondition::Or(Box::new(self), Box::new(other))
    }

    /// Build [QueryCondition::Not] from self
    pub fn not(self) -> QueryCondition<T> {
        QueryCondition::Not(Box::new(self))
    }
}
