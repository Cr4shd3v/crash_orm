use std::marker::PhantomData;
use async_trait::async_trait;
use tokio_postgres::types::ToSql;
use crate::{DatabaseConnection, Entity};

#[async_trait]
pub trait QueryEntity<T: Entity + Send + 'static>: Entity {
    async fn query(connection: &DatabaseConnection, condition: QueryCondition<T>) -> crate::Result<Vec<Self::Output>> {
        let (query, values, _) = condition.resolve(1);

        let rows = connection.query(
            &*format!("SELECT * FROM {} WHERE {}", Self::TABLE_NAME, query),
            slice_iter(values.as_slice()).collect::<Vec<&(dyn ToSql + Sync)>>().as_slice(),
        ).await?;

        Ok(rows.iter().map(|r| Self::load_from_row(r)).collect())
    }

    async fn count_query(connection: &DatabaseConnection, condition: QueryCondition<T>) -> crate::Result<i64> {
        let (query, values, _) = condition.resolve(1);

        let row = connection.query_one(
            &*format!("SELECT COUNT(*) FROM {} WHERE {}", Self::TABLE_NAME, query),
            slice_iter(values.as_slice()).collect::<Vec<&(dyn ToSql + Sync)>>().as_slice(),
        ).await?;

        Ok(row.get(0))
    }

    async fn count_column<U: ToSql + Send>(connection: &DatabaseConnection, column: EntityColumn<U, T>, distinct: bool) -> crate::Result<i64> {
        let row = connection.query_one(
            &*format!("SELECT COUNT({}{}) FROM {}", if distinct { "DISTINCT " } else { "" }, column.name, Self::TABLE_NAME),
            &[],
        ).await?;

        Ok(row.get(0))
    }

    async fn count_column_query<U: ToSql + Send>(connection: &DatabaseConnection, column: EntityColumn<U, T>, distinct: bool, condition: QueryCondition<T>) -> crate::Result<i64> {
        let (query, values, _) = condition.resolve(1);

        let row = connection.query_one(
            &*format!("SELECT COUNT({}{}) FROM {} WHERE {}", if distinct { "DISTINCT " } else { "" }, column.name, Self::TABLE_NAME, query),
            slice_iter(values.as_slice()).collect::<Vec<&(dyn ToSql + Sync)>>().as_slice(),
        ).await?;

        Ok(row.get(0))
    }
}

fn slice_iter<'a>(
    s: &'a [Box<dyn ToSql + Send + Sync>],
) -> impl ExactSizeIterator<Item = &'a (dyn ToSql + Sync)> + 'a {
    s.iter().map(|s| &**s as _)
}

pub enum QueryCondition<T: Entity + Send> {
    Equals(String, Box<dyn ToSql + Sync + Send>),
    NotEquals(String, Box<dyn ToSql + Sync + Send>),
    And(Box<QueryCondition<T>>, Box<QueryCondition<T>>),
    Or(Box<QueryCondition<T>>, Box<QueryCondition<T>>),
    IsNull(String),
    IsNotNull(String),
    #[allow(non_camel_case_types)]__(PhantomData<T>),
}

impl<T: Entity + Send> QueryCondition<T> {
    fn resolve(self, index: usize) -> (String, Vec<Box<dyn ToSql + Send + Sync>>, usize) {
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
        }
    }

    pub fn and(self, other: QueryCondition<T>) -> QueryCondition<T> {
        QueryCondition::And(Box::new(self), Box::new(other))
    }

    pub fn or(self, other: QueryCondition<T>) -> QueryCondition<T> {
        QueryCondition::Or(Box::new(self), Box::new(other))
    }
}

pub struct EntityColumn<T: ToSql, U: Entity> {
    name: &'static str,
    phantom_1: PhantomData<T>,
    phantom_2: PhantomData<U>,
}

impl<T: ToSql, U: Entity + Send> EntityColumn<T, U> {
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

pub trait NullQueryColumn<T: ToSql, U: Entity + Send> {
    fn is_null(&self) -> QueryCondition<U>;

    fn is_not_null(&self) -> QueryCondition<U>;
}

impl<T: ToSql, U: Entity + Send> NullQueryColumn<T, U> for EntityColumn<Option<T>, U>  {
    fn is_null(&self) -> QueryCondition<U> {
        QueryCondition::IsNull(self.name.to_string())
    }

    fn is_not_null(&self) -> QueryCondition<U> {
        QueryCondition::IsNotNull(self.name.to_string())
    }
}

pub trait EqualQueryColumn<T: ToSql, U: Entity + Send> {
    fn equals(&self, other: T) -> QueryCondition<U>;

    fn not_equals(&self, other: T) -> QueryCondition<U>;
}

macro_rules! impl_equal_entity_column {
    ($column_type:ty) => {
        impl<T: Entity + Send> EqualQueryColumn<$column_type, T> for EntityColumn<$column_type, T> {
            fn equals(&self, other: $column_type) -> QueryCondition<T> {
                QueryCondition::Equals(self.name.to_string(), Box::new(other))
            }

            fn not_equals(&self, other: $column_type) -> QueryCondition<T> {
                QueryCondition::NotEquals(self.name.to_string(), Box::new(other))
            }
        }

        impl<T: Entity + Send> EqualQueryColumn<$column_type, T> for EntityColumn<Option<$column_type>, T> {
            fn equals(&self, other: $column_type) -> QueryCondition<T> {
                QueryCondition::Equals(self.name.to_string(), Box::new(other))
            }

            fn not_equals(&self, other: $column_type) -> QueryCondition<T> {
                QueryCondition::NotEquals(self.name.to_string(), Box::new(other))
            }
        }
    };
}

impl_equal_entity_column!(bool);
impl_equal_entity_column!(i8);
impl_equal_entity_column!(i16);
impl_equal_entity_column!(i32);
impl_equal_entity_column!(i64);
impl_equal_entity_column!(u32);
impl_equal_entity_column!(f32);
impl_equal_entity_column!(f64);
impl_equal_entity_column!(String);