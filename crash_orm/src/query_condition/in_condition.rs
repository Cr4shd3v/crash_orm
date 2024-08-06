use tokio_postgres::types::ToSql;

use crate::prelude::{Column, Entity, IntoSql, PrimaryKey, QueryCondition};

/// Trait implementing IN operator [QueryCondition]
pub trait InQueryColumn<T: ToSql, U: Entity<U, P>, P: PrimaryKey> {
    /// Creates [QueryCondition::In] from self and other
    fn in_vec(&self, other: Vec<impl IntoSql<T>>) -> QueryCondition<U, P>;

    /// Creates [QueryCondition::NotIn] from self and other
    fn not_in_vec(&self, other: Vec<impl IntoSql<T>>) -> QueryCondition<U, P>;
}

macro_rules! impl_in_entity_column {
    ($column_type:ty) => {
        impl<U: Entity<U, P>, R: Column<$column_type, U, P>, P: PrimaryKey> InQueryColumn<$column_type, U, P> for R {
            fn in_vec(
                &self,
                other: Vec<impl IntoSql<$column_type>>,
            ) -> QueryCondition<U, P> {
                QueryCondition::In(self.get_sql(), other.iter().map(|i| i.into_typed_value().get_sql()).collect())
            }

            fn not_in_vec(
                &self,
                other: Vec<impl IntoSql<$column_type>>,
            ) -> QueryCondition<U, P> {
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
