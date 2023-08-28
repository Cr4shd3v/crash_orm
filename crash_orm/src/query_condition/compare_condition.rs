use rust_decimal::Decimal;
use tokio_postgres::types::ToSql;
use crate::{Entity, Column, QueryCondition};

pub trait CompareQueryColumn<T: ToSql, U: Entity<U>> {
    fn greater_than(&self, other: T) -> QueryCondition<U>;
    fn greater_equal(&self, other: T) -> QueryCondition<U>;
    fn less_than(&self, other: T) -> QueryCondition<U>;
    fn less_equal(&self, other: T) -> QueryCondition<U>;
    fn between(&self, from: T, to: T) -> QueryCondition<U>;
    fn not_between(&self, from: T, to: T) -> QueryCondition<U>;
}

macro_rules! impl_compare_entity_column {
    ($column_type:ty) => {
        impl<U: Entity<U>, R: Column<$column_type, U>> CompareQueryColumn<$column_type, U> for R {
            fn greater_than(&self, other: $column_type) -> QueryCondition<U> {
                QueryCondition::GreaterThan(self.get_name(), Box::new(other))
            }

            fn greater_equal(&self, other: $column_type) -> QueryCondition<U> {
                QueryCondition::GreaterEqual(self.get_name(), Box::new(other))
            }

            fn less_than(&self, other: $column_type) -> QueryCondition<U> {
                QueryCondition::LessThan(self.get_name(), Box::new(other))
            }

            fn less_equal(&self, other: $column_type) -> QueryCondition<U> {
                QueryCondition::LessEqual(self.get_name(), Box::new(other))
            }

            fn between(&self, from: $column_type, to: $column_type) -> QueryCondition<U> {
                QueryCondition::Between(self.get_name(), Box::new(from), Box::new(to))
            }

            fn not_between(&self, from: $column_type, to: $column_type) -> QueryCondition<U> {
                QueryCondition::NotBetween(self.get_name(), Box::new(from), Box::new(to))
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