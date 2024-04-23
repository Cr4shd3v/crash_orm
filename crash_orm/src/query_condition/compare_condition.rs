use tokio_postgres::types::ToSql;

use crate::{Column, Entity, IntoSql, PrimaryKey, QueryCondition};

/// Trait implementing comparison operator [QueryCondition]
pub trait CompareQueryColumn<T: ToSql, U: Entity<U, P>, P: PrimaryKey> {
    /// Creates [QueryCondition::GreaterThan] from self and other
    fn greater_than(&self, other: impl IntoSql<T>) -> QueryCondition<U, P>;

    /// Creates [QueryCondition::GreaterEqual] from self and other
    fn greater_equal(&self, other: impl IntoSql<T>) -> QueryCondition<U, P>;

    /// Creates [QueryCondition::LessThan] from self and other
    fn less_than(&self, other: impl IntoSql<T>) -> QueryCondition<U, P>;

    /// Creates [QueryCondition::LessEqual] from self and other
    fn less_equal(&self, other: impl IntoSql<T>) -> QueryCondition<U, P>;

    /// Creates [QueryCondition::Between] from self and from - to
    fn between(
        &self,
        from: impl IntoSql<T>,
        to: impl IntoSql<T>,
    ) -> QueryCondition<U, P>;

    /// Creates [QueryCondition::NotBetween] from self and from - to
    fn not_between(
        &self,
        from: impl IntoSql<T>,
        to: impl IntoSql<T>,
    ) -> QueryCondition<U, P>;
}

macro_rules! impl_compare_entity_column {
    ($column_type:ty) => {
        impl<U: Entity<U, P>, R: Column<$column_type, U, P>, P: PrimaryKey> CompareQueryColumn<$column_type, U, P> for R {
            fn greater_than(
                &self,
                other: impl IntoSql<$column_type>,
            ) -> QueryCondition<U, P> {
                QueryCondition::GreaterThan(self.get_sql(), other.into_typed_value().get_sql())
            }

            fn greater_equal(
                &self,
                other: impl IntoSql<$column_type>,
            ) -> QueryCondition<U, P> {
                QueryCondition::GreaterEqual(self.get_sql(), other.into_typed_value().get_sql())
            }

            fn less_than(&self, other: impl IntoSql<$column_type>) -> QueryCondition<U, P> {
                QueryCondition::LessThan(self.get_sql(), other.into_typed_value().get_sql())
            }

            fn less_equal(
                &self,
                other: impl IntoSql<$column_type>,
            ) -> QueryCondition<U, P> {
                QueryCondition::LessEqual(self.get_sql(), other.into_typed_value().get_sql())
            }

            fn between(
                &self,
                from: impl IntoSql<$column_type>,
                to: impl IntoSql<$column_type>,
            ) -> QueryCondition<U, P> {
                QueryCondition::Between(self.get_sql(), from.into_typed_value().get_sql(), to.into_typed_value().get_sql())
            }

            fn not_between(
                &self,
                from: impl IntoSql<$column_type>,
                to: impl IntoSql<$column_type>,
            ) -> QueryCondition<U, P> {
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
