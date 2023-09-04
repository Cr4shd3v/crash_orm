use rust_decimal::Decimal;
use tokio_postgres::types::ToSql;
use crate::{Entity, Column, QueryCondition, TypedColumnValue};

pub trait CompareQueryColumn<T: ToSql, U: Entity<U>> {
    fn greater_than(&self, other: &(dyn TypedColumnValue<T>)) -> QueryCondition<U>;
    fn greater_equal(&self, other: &(dyn TypedColumnValue<T>)) -> QueryCondition<U>;
    fn less_than(&self, other: &(dyn TypedColumnValue<T>)) -> QueryCondition<U>;
    fn less_equal(&self, other: &(dyn TypedColumnValue<T>)) -> QueryCondition<U>;
    fn between(&self, from: &(dyn TypedColumnValue<T>), to: &(dyn TypedColumnValue<T>)) -> QueryCondition<U>;
    fn not_between(&self, from: &(dyn TypedColumnValue<T>), to: &(dyn TypedColumnValue<T>)) -> QueryCondition<U>;
}

macro_rules! impl_compare_entity_column {
    ($column_type:ty) => {
        impl<U: Entity<U>, R: Column<$column_type, U>> CompareQueryColumn<$column_type, U> for R {
            fn greater_than(&self, other: &(dyn TypedColumnValue<$column_type>)) -> QueryCondition<U> {
                QueryCondition::GreaterThan(self.get_sql(), other.get_sql())
            }

            fn greater_equal(&self, other: &(dyn TypedColumnValue<$column_type>)) -> QueryCondition<U> {
                QueryCondition::GreaterEqual(self.get_sql(), other.get_sql())
            }

            fn less_than(&self, other: &(dyn TypedColumnValue<$column_type>)) -> QueryCondition<U> {
                QueryCondition::LessThan(self.get_sql(), other.get_sql())
            }

            fn less_equal(&self, other: &(dyn TypedColumnValue<$column_type>)) -> QueryCondition<U> {
                QueryCondition::LessEqual(self.get_sql(), other.get_sql())
            }

            fn between(&self, from: &(dyn TypedColumnValue<$column_type>), to: &(dyn TypedColumnValue<$column_type>)) -> QueryCondition<U> {
                QueryCondition::Between(self.get_sql(), from.get_sql(), to.get_sql())
            }

            fn not_between(&self, from: &(dyn TypedColumnValue<$column_type>), to: &(dyn TypedColumnValue<$column_type>)) -> QueryCondition<U> {
                QueryCondition::NotBetween(self.get_sql(), from.get_sql(), to.get_sql())
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