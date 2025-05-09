use std::fmt::Debug;
use crate::prelude::{Column, ColumnType, Entity, IntoSql, QueryCondition, TypedJson};

/// Trait implementing equals operator [QueryCondition]
pub trait EqualQueryColumn<T: ColumnType, U: Entity> {
    /// Creates [QueryCondition::Equals] from self and other
    fn equals(&self, other: impl IntoSql<T>) -> QueryCondition<U>;

    /// Creates [QueryCondition::NotEquals] from self and other
    fn not_equals(&self, other: impl IntoSql<T>) -> QueryCondition<U>;
}

macro_rules! impl_equal_entity_column {
    ($column_type:ty) => {
        impl<T: Entity, U: Column<$column_type, T>> EqualQueryColumn<$column_type, T> for U {
            fn equals(&self, other: impl IntoSql<$column_type>) -> QueryCondition<T> {
                let mut boxed = self.get_sql();
                let other_boxed = other.into_boxed_sql();
                boxed.modify(|v| format!("{v} = {}", other_boxed.sql));
                boxed.values.extend(other_boxed.values);

                QueryCondition::new(boxed)
            }

            fn not_equals(
                &self,
                other: impl IntoSql<$column_type>,
            ) -> QueryCondition<T> {
                let mut boxed = self.get_sql();
                let other_boxed = other.into_boxed_sql();
                boxed.modify(|v| format!("{v} <> {}", other_boxed.sql));
                boxed.values.extend(other_boxed.values);

                QueryCondition::new(boxed)
            }
        }
    };
}

#[cfg(feature = "with-geo-types")]
macro_rules! impl_equal_entity_column_geo {
    ($column_type:ty) => {
        impl<T: Entity, U: Column<$column_type, T>> EqualQueryColumn<$column_type, T> for U {
            fn equals(&self, other: impl IntoSql<$column_type>) -> QueryCondition<T> {
                let mut boxed = self.get_sql();
                let other_boxed = other.into_boxed_sql();
                boxed.modify(|v| format!("{v} ~= {}", other_boxed.sql));
                boxed.values.extend(other_boxed.values);

                QueryCondition::new(boxed)
            }

            fn not_equals(
                &self,
                other: impl IntoSql<$column_type>,
            ) -> QueryCondition<T> {
                self.equals(other).not()
            }
        }
    };
}

#[cfg(feature = "json")]
impl<JSON: serde::Serialize + serde::de::DeserializeOwned + Debug + Clone + Send + Sync + 'static, T: Entity, U: Column<TypedJson<JSON>, T>> EqualQueryColumn<TypedJson<JSON>, T> for U {
    fn equals(&self, other: impl IntoSql<TypedJson<JSON>>) -> QueryCondition<T> {
        let mut boxed = self.get_sql();
        let other_boxed = other.into_boxed_sql();
        boxed.modify(|v| format!("{v} = {}", other_boxed.sql));
        boxed.values.extend(other_boxed.values);

        QueryCondition::new(boxed)
    }

    fn not_equals(
        &self,
        other: impl IntoSql<TypedJson<JSON>>,
    ) -> QueryCondition<T> {
        let mut boxed = self.get_sql();
        let other_boxed = other.into_boxed_sql();
        boxed.modify(|v| format!("{v} <> {}", other_boxed.sql));
        boxed.values.extend(other_boxed.values);

        QueryCondition::new(boxed)
    }
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
#[cfg(feature = "json")]
impl_equal_entity_column!(serde_json::Value);
#[cfg(feature = "with-eui48")]
impl_equal_entity_column!(eui48::MacAddress);
#[cfg(feature = "with-time")]
impl_equal_entity_column!(time::PrimitiveDateTime);
#[cfg(feature = "with-time")]
impl_equal_entity_column!(time::OffsetDateTime);
#[cfg(feature = "with-time")]
impl_equal_entity_column!(time::Date);
#[cfg(feature = "with-time")]
impl_equal_entity_column!(time::Time);
#[cfg(feature = "with-geo-types")]
impl_equal_entity_column_geo!(geo_types::Point);
#[cfg(feature = "with-geo-types")]
impl_equal_entity_column_geo!(geo_types::Rect);
#[cfg(feature = "with-geo-types")]
impl_equal_entity_column_geo!(geo_types::LineString);