use tokio_postgres::types::ToSql;

use crate::{Column, Entity, IntoSql, PrimaryKey, QueryCondition};

/// Trait implementing equals operator [QueryCondition]
pub trait EqualQueryColumn<T: ToSql, U: Entity<U, P>, P: PrimaryKey> {
    /// Creates [QueryCondition::Equals] from self and other
    fn equals(&self, other: impl IntoSql<T>) -> QueryCondition<U, P>;

    /// Creates [QueryCondition::NotEquals] from self and other
    fn not_equals(&self, other: impl IntoSql<T>) -> QueryCondition<U, P>;
}

macro_rules! impl_equal_entity_column {
    ($column_type:ty) => {
        impl<T: Entity<T, P>, U: Column<$column_type, T, P>, P: PrimaryKey> EqualQueryColumn<$column_type, T, P> for U {
            fn equals(&self, other: impl IntoSql<$column_type>) -> QueryCondition<T, P> {
                QueryCondition::Equals(self.get_sql(), other.into_typed_value().get_sql())
            }

            fn not_equals(
                &self,
                other: impl IntoSql<$column_type>,
            ) -> QueryCondition<T, P> {
                QueryCondition::NotEquals(self.get_sql(), other.into_typed_value().get_sql())
            }
        }
    };
}

impl_equal_entity_column!(bool);
impl_equal_entity_column!(i8);
impl_equal_entity_column!(i16);
impl_equal_entity_column!(i32);
impl_equal_entity_column!(i64);
#[cfg(feature = "with-rust-decimal")]
impl_equal_entity_column!(rust_decimal::Decimal);
impl_equal_entity_column!(u32);
impl_equal_entity_column!(f32);
impl_equal_entity_column!(f64);
impl_equal_entity_column!(String);
#[cfg(feature = "with-chrono")]
impl_equal_entity_column!(chrono::NaiveDateTime);
#[cfg(feature = "with-chrono")]
impl_equal_entity_column!(chrono::DateTime<chrono::Utc>);
#[cfg(feature = "with-chrono")]
impl_equal_entity_column!(chrono::DateTime<chrono::Local>);
#[cfg(feature = "with-chrono")]
impl_equal_entity_column!(chrono::DateTime<chrono::FixedOffset>);
#[cfg(feature = "with-chrono")]
impl_equal_entity_column!(chrono::NaiveDate);
#[cfg(feature = "with-chrono")]
impl_equal_entity_column!(chrono::NaiveTime);
#[cfg(feature = "with-uuid")]
impl_equal_entity_column!(uuid::Uuid);
impl_equal_entity_column!(Vec<u8>);
#[cfg(feature = "with-serde")]
impl_equal_entity_column!(serde_json::Value);
#[cfg(feature = "with-eui48")]
impl_equal_entity_column!(eui48::MacAddress);
#[cfg(feature = "with-bit-vec")]
impl_equal_entity_column!(bit_vec::BitVec);
