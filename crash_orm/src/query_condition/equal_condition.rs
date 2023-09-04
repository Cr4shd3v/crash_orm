use chrono::*;
use rust_decimal::Decimal;
use serde_json::Value;
use tokio_postgres::types::ToSql;
use uuid::Uuid;
use crate::{Entity, QueryCondition, Column, TypedColumnValue};

pub trait EqualQueryColumn<T: ToSql, U: Entity<U>> {
    fn equals(&self, other: &(dyn TypedColumnValue<T>)) -> QueryCondition<U>;

    fn not_equals(&self, other: &(dyn TypedColumnValue<T>)) -> QueryCondition<U>;
}

macro_rules! impl_equal_entity_column {
    ($column_type:ty) => {
        impl<T: Entity<T>, U: Column<$column_type, T>> EqualQueryColumn<$column_type, T> for U {
            fn equals(&self, other: &(dyn TypedColumnValue<$column_type>)) -> QueryCondition<T> {
                QueryCondition::Equals(self.get_sql(), other.get_sql())
            }

            fn not_equals(&self, other: &(dyn TypedColumnValue<$column_type>)) -> QueryCondition<T> {
                QueryCondition::NotEquals(self.get_sql(), other.get_sql())
            }
        }
    };
}

impl_equal_entity_column!(bool);
impl_equal_entity_column!(i8);
impl_equal_entity_column!(i16);
impl_equal_entity_column!(i32);
impl_equal_entity_column!(i64);
impl_equal_entity_column!(Decimal);
impl_equal_entity_column!(u32);
impl_equal_entity_column!(f32);
impl_equal_entity_column!(f64);
impl_equal_entity_column!(String);
impl_equal_entity_column!(NaiveDateTime);
impl_equal_entity_column!(DateTime<Utc>);
impl_equal_entity_column!(DateTime<Local>);
impl_equal_entity_column!(DateTime<FixedOffset>);
impl_equal_entity_column!(NaiveDate);
impl_equal_entity_column!(NaiveTime);
impl_equal_entity_column!(Uuid);
impl_equal_entity_column!(Vec<u8>);
impl_equal_entity_column!(Value);