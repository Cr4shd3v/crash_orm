use rust_decimal::Decimal;
use tokio_postgres::types::ToSql;
use crate::{Entity, Column, QueryCondition};

pub trait InQueryColumn<T: ToSql, U: Entity<U>> {
    fn r#in(&self, other: Vec<T>) -> QueryCondition<U>;

    fn not_in(&self, other: Vec<T>) -> QueryCondition<U>;
}

macro_rules! impl_in_entity_column {
    ($column_type:ty) => {
        impl<U: Entity<U>, R: Column<$column_type, U>> InQueryColumn<$column_type, U> for R {
            fn r#in(&self, other: Vec<$column_type>) -> QueryCondition<U> {
                QueryCondition::In(
                    self.get_name(),
                    other.iter().map(|i| -> Box<dyn ToSql + Sync + Send>{Box::new((*i).clone())}).collect()
                )
            }

            fn not_in(&self, other: Vec<$column_type>) -> QueryCondition<U> {
                QueryCondition::NotIn(
                    self.get_name(),
                    other.iter().map(|i| -> Box<dyn ToSql + Sync + Send>{Box::new((*i).clone())}).collect()
                )
            }
        }
    };
}

impl_in_entity_column!(i8);
impl_in_entity_column!(i16);
impl_in_entity_column!(i32);
impl_in_entity_column!(i64);
impl_in_entity_column!(Decimal);
impl_in_entity_column!(f32);
impl_in_entity_column!(f64);
impl_in_entity_column!(String);