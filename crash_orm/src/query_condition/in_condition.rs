use tokio_postgres::types::ToSql;
use crate::{Entity, Column, QueryCondition, TypedColumnValue};

/// Trait implementing IN operators
pub trait InQueryColumn<T: ToSql, U: Entity<U>> {
    fn in_vec(&self, other: Vec<&(dyn TypedColumnValue<T>)>) -> QueryCondition<U>;

    fn not_in_vec(&self, other: Vec<&(dyn TypedColumnValue<T>)>) -> QueryCondition<U>;
}

macro_rules! impl_in_entity_column {
    ($column_type:ty) => {
        impl<U: Entity<U>, R: Column<$column_type, U>> InQueryColumn<$column_type, U> for R {
            fn in_vec(&self, other: Vec<&(dyn TypedColumnValue<$column_type>)>) -> QueryCondition<U> {
                QueryCondition::In(
                    self.get_sql(),
                    other.iter().map(|i| i.get_sql()).collect()
                )
            }

            fn not_in_vec(&self, other: Vec<&(dyn TypedColumnValue<$column_type>)>) -> QueryCondition<U> {
                QueryCondition::NotIn(
                    self.get_sql(),
                    other.iter().map(|i| i.get_sql()).collect()
                )
            }
        }
    };
}

impl_in_entity_column!(i8);
impl_in_entity_column!(i16);
impl_in_entity_column!(i32);
impl_in_entity_column!(i64);
#[cfg(feature = "with-rust-decimal")]
impl_in_entity_column!(rust_decimal::Decimal);
impl_in_entity_column!(f32);
impl_in_entity_column!(f64);
impl_in_entity_column!(String);