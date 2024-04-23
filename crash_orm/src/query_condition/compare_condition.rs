use tokio_postgres::types::ToSql;

use crate::{Column, Entity, IntoSql, PrimaryKey, QueryCondition};

/// Trait implementing comparison operators
pub trait CompareQueryColumn<T: ToSql, U: Entity<U, PRIMARY>, PRIMARY: PrimaryKey> {
    fn greater_than(&self, other: impl IntoSql<T>) -> QueryCondition<U, PRIMARY>;
    fn greater_equal(&self, other: impl IntoSql<T>) -> QueryCondition<U, PRIMARY>;
    fn less_than(&self, other: impl IntoSql<T>) -> QueryCondition<U, PRIMARY>;
    fn less_equal(&self, other: impl IntoSql<T>) -> QueryCondition<U, PRIMARY>;
    fn between(
        &self,
        from: impl IntoSql<T>,
        to: impl IntoSql<T>,
    ) -> QueryCondition<U, PRIMARY>;
    fn not_between(
        &self,
        from: impl IntoSql<T>,
        to: impl IntoSql<T>,
    ) -> QueryCondition<U, PRIMARY>;
}

macro_rules! impl_compare_entity_column {
    ($column_type:ty) => {
        impl<U: Entity<U, PRIMARY>, R: Column<$column_type, U, PRIMARY>, PRIMARY: PrimaryKey> CompareQueryColumn<$column_type, U, PRIMARY> for R {
            fn greater_than(
                &self,
                other: impl IntoSql<$column_type>,
            ) -> QueryCondition<U, PRIMARY> {
                QueryCondition::GreaterThan(self.get_sql(), other.into_typed_value().get_sql())
            }

            fn greater_equal(
                &self,
                other: impl IntoSql<$column_type>,
            ) -> QueryCondition<U, PRIMARY> {
                QueryCondition::GreaterEqual(self.get_sql(), other.into_typed_value().get_sql())
            }

            fn less_than(&self, other: impl IntoSql<$column_type>) -> QueryCondition<U, PRIMARY> {
                QueryCondition::LessThan(self.get_sql(), other.into_typed_value().get_sql())
            }

            fn less_equal(
                &self,
                other: impl IntoSql<$column_type>,
            ) -> QueryCondition<U, PRIMARY> {
                QueryCondition::LessEqual(self.get_sql(), other.into_typed_value().get_sql())
            }

            fn between(
                &self,
                from: impl IntoSql<$column_type>,
                to: impl IntoSql<$column_type>,
            ) -> QueryCondition<U, PRIMARY> {
                QueryCondition::Between(self.get_sql(), from.into_typed_value().get_sql(), to.into_typed_value().get_sql())
            }

            fn not_between(
                &self,
                from: impl IntoSql<$column_type>,
                to: impl IntoSql<$column_type>,
            ) -> QueryCondition<U, PRIMARY> {
                QueryCondition::NotBetween(self.get_sql(), from.into_typed_value().get_sql(), to.into_typed_value().get_sql())
            }
        }
    };
}

impl_compare_entity_column!(i8);
impl_compare_entity_column!(i16);
impl_compare_entity_column!(i32);
impl_compare_entity_column!(i64);
#[cfg(feature = "with-rust-decimal")]
impl_compare_entity_column!(rust_decimal::Decimal);
impl_compare_entity_column!(f32);
impl_compare_entity_column!(f64);
#[cfg(feature = "with-chrono")]
impl_compare_entity_column!(chrono::NaiveDateTime);
#[cfg(feature = "with-chrono")]
impl_compare_entity_column!(chrono::DateTime<chrono::Utc>);
#[cfg(feature = "with-chrono")]
impl_compare_entity_column!(chrono::DateTime<chrono::Local>);
#[cfg(feature = "with-chrono")]
impl_compare_entity_column!(chrono::DateTime<chrono::FixedOffset>);
#[cfg(feature = "with-chrono")]
impl_compare_entity_column!(chrono::NaiveDate);
#[cfg(feature = "with-chrono")]
impl_compare_entity_column!(chrono::NaiveTime);
