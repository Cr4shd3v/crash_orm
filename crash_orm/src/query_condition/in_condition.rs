use tokio_postgres::types::ToSql;

use crate::{Column, Entity, IntoSql, PrimaryKey, QueryCondition};

/// Trait implementing IN operators
pub trait InQueryColumn<T: ToSql, U: Entity<U, PRIMARY>, PRIMARY: PrimaryKey<'static>> {
    fn in_vec(&self, other: Vec<impl IntoSql<T>>) -> QueryCondition<U, PRIMARY>;

    fn not_in_vec(&self, other: Vec<impl IntoSql<T>>) -> QueryCondition<U, PRIMARY>;
}

macro_rules! impl_in_entity_column {
    ($column_type:ty) => {
        impl<U: Entity<U, PRIMARY>, R: Column<$column_type, U, PRIMARY>, PRIMARY: PrimaryKey<'static>> InQueryColumn<$column_type, U, PRIMARY> for R {
            fn in_vec(
                &self,
                other: Vec<impl IntoSql<$column_type>>,
            ) -> QueryCondition<U, PRIMARY> {
                QueryCondition::In(self.get_sql(), other.iter().map(|i| i.into_typed_value().get_sql()).collect())
            }

            fn not_in_vec(
                &self,
                other: Vec<impl IntoSql<$column_type>>,
            ) -> QueryCondition<U, PRIMARY> {
                QueryCondition::NotIn(self.get_sql(), other.iter().map(|i| i.into_typed_value().get_sql()).collect())
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
