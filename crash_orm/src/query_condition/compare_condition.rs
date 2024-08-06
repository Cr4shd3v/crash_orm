use tokio_postgres::types::ToSql;

use crate::prelude::{Column, Entity, IntoSql, QueryCondition};

/// Trait implementing comparison operator [QueryCondition]
pub trait CompareQueryColumn<T: ToSql, U: Entity<U>> {
    /// Creates [QueryCondition::GreaterThan] from self and other
    fn greater_than(&self, other: impl IntoSql<T>) -> QueryCondition<U>;

    /// Creates [QueryCondition::GreaterEqual] from self and other
    fn greater_equal(&self, other: impl IntoSql<T>) -> QueryCondition<U>;

    /// Creates [QueryCondition::LessThan] from self and other
    fn less_than(&self, other: impl IntoSql<T>) -> QueryCondition<U>;

    /// Creates [QueryCondition::LessEqual] from self and other
    fn less_equal(&self, other: impl IntoSql<T>) -> QueryCondition<U>;

    /// Creates [QueryCondition::Between] from self and from - to
    fn between(
        &self,
        from: impl IntoSql<T>,
        to: impl IntoSql<T>,
    ) -> QueryCondition<U>;

    /// Creates [QueryCondition::NotBetween] from self and from - to
    fn not_between(
        &self,
        from: impl IntoSql<T>,
        to: impl IntoSql<T>,
    ) -> QueryCondition<U>;
}

macro_rules! impl_compare_entity_column {
    ($column_type:ty) => {
        impl<U: Entity<U>, R: Column<$column_type, U>> CompareQueryColumn<$column_type, U> for R {
            fn greater_than(
                &self,
                other: impl IntoSql<$column_type>,
            ) -> QueryCondition<U> {
                let mut boxed = self.get_sql();
                let other_boxed = other.into_boxed_sql();
                boxed.modify(|v| format!("{v} > {}", other_boxed.sql));
                boxed.values.extend(other_boxed.values);

                QueryCondition::new(boxed)
            }

            fn greater_equal(
                &self,
                other: impl IntoSql<$column_type>,
            ) -> QueryCondition<U> {
                let mut boxed = self.get_sql();
                let other_boxed = other.into_boxed_sql();
                boxed.modify(|v| format!("{v} >= {}", other_boxed.sql));
                boxed.values.extend(other_boxed.values);

                QueryCondition::new(boxed)
            }

            fn less_than(&self, other: impl IntoSql<$column_type>) -> QueryCondition<U> {
                let mut boxed = self.get_sql();
                let other_boxed = other.into_boxed_sql();
                boxed.modify(|v| format!("{v} < {}", other_boxed.sql));
                boxed.values.extend(other_boxed.values);

                QueryCondition::new(boxed)
            }

            fn less_equal(
                &self,
                other: impl IntoSql<$column_type>,
            ) -> QueryCondition<U> {
                let mut boxed = self.get_sql();
                let other_boxed = other.into_boxed_sql();
                boxed.modify(|v| format!("{v} <= {}", other_boxed.sql));
                boxed.values.extend(other_boxed.values);

                QueryCondition::new(boxed)
            }

            fn between(
                &self,
                from: impl IntoSql<$column_type>,
                to: impl IntoSql<$column_type>,
            ) -> QueryCondition<U> {
                let mut boxed = self.get_sql();
                let from_boxed = from.into_boxed_sql();
                let to_boxed = to.into_boxed_sql();
                boxed.modify(|v| format!("{v} BETWEEN {} AND {}", from_boxed.sql, to_boxed.sql));
                boxed.values.extend(from_boxed.values);
                boxed.values.extend(to_boxed.values);

                QueryCondition::new(boxed)
            }

            fn not_between(
                &self,
                from: impl IntoSql<$column_type>,
                to: impl IntoSql<$column_type>,
            ) -> QueryCondition<U> {
                let mut boxed = self.get_sql();
                let from_boxed = from.into_boxed_sql();
                let to_boxed = to.into_boxed_sql();
                boxed.modify(|v| format!("{v} NOT BETWEEN {} AND {}", from_boxed.sql, to_boxed.sql));
                boxed.values.extend(from_boxed.values);
                boxed.values.extend(to_boxed.values);

                QueryCondition::new(boxed)
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
#[cfg(feature = "with-time")]
impl_compare_entity_column!(time::PrimitiveDateTime);
#[cfg(feature = "with-time")]
impl_compare_entity_column!(time::OffsetDateTime);
#[cfg(feature = "with-time")]
impl_compare_entity_column!(time::Date);
#[cfg(feature = "with-time")]
impl_compare_entity_column!(time::Time);