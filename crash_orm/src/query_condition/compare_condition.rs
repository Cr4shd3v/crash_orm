use rust_decimal::Decimal;
use tokio_postgres::types::ToSql;
use crate::{Entity, EntityColumn, QueryCondition};

pub trait CompareQueryColumn<T: ToSql, U: Entity<U> + Send + 'static> {
    fn greater_than(&self, other: T) -> QueryCondition<U>;
    fn greater_equal(&self, other: T) -> QueryCondition<U>;
    fn less_than(&self, other: T) -> QueryCondition<U>;
    fn less_equal(&self, other: T) -> QueryCondition<U>;
}

macro_rules! impl_compare_entity_column {
    ($column_type:ty) => {
        impl<U: Entity<U> + Send + 'static> CompareQueryColumn<$column_type, U> for EntityColumn<$column_type, U> {
            fn greater_than(&self, other: $column_type) -> QueryCondition<U> {
                QueryCondition::GreaterThan(self.name.to_string(), Box::new(other))
            }

            fn greater_equal(&self, other: $column_type) -> QueryCondition<U> {
                QueryCondition::GreaterEqual(self.name.to_string(), Box::new(other))
            }

            fn less_than(&self, other: $column_type) -> QueryCondition<U> {
                QueryCondition::LessThan(self.name.to_string(), Box::new(other))
            }

            fn less_equal(&self, other: $column_type) -> QueryCondition<U> {
                QueryCondition::LessEqual(self.name.to_string(), Box::new(other))
            }
        }

        impl<U: Entity<U> + Send + 'static> CompareQueryColumn<$column_type, U> for EntityColumn<Option<$column_type>, U> {
            fn greater_than(&self, other: $column_type) -> QueryCondition<U> {
                QueryCondition::GreaterThan(self.name.to_string(), Box::new(other))
            }

            fn greater_equal(&self, other: $column_type) -> QueryCondition<U> {
                QueryCondition::GreaterEqual(self.name.to_string(), Box::new(other))
            }

            fn less_than(&self, other: $column_type) -> QueryCondition<U> {
                QueryCondition::LessThan(self.name.to_string(), Box::new(other))
            }

            fn less_equal(&self, other: $column_type) -> QueryCondition<U> {
                QueryCondition::LessEqual(self.name.to_string(), Box::new(other))
            }
        }
    };
}

impl_compare_entity_column!(i8);
impl_compare_entity_column!(i16);
impl_compare_entity_column!(i32);
impl_compare_entity_column!(i64);
impl_compare_entity_column!(Decimal);
impl_compare_entity_column!(f32);
impl_compare_entity_column!(f64);