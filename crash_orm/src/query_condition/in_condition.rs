use tokio_postgres::types::ToSql;

use crate::prelude::{BoxedSql, Column, Entity, IntoSql, QueryCondition};

/// Trait implementing IN operator [QueryCondition]
pub trait InQueryColumn<T: ToSql, U: Entity<U>> {
    /// Creates [QueryCondition::In] from self and other
    fn in_vec(&self, other: Vec<impl IntoSql<T>>) -> QueryCondition<U>;

    /// Creates [QueryCondition::NotIn] from self and other
    fn not_in_vec(&self, other: Vec<impl IntoSql<T>>) -> QueryCondition<U>;
}

macro_rules! impl_in_entity_column {
    ($column_type:ty) => {
        impl<U: Entity<U>, R: Column<$column_type, U>> InQueryColumn<$column_type, U> for R {
            fn in_vec(
                &self,
                other: Vec<impl IntoSql<$column_type>>,
            ) -> QueryCondition<U> {
                let mut boxed = self.get_sql();
                let other_boxed = other.iter().map(|i| i.into_boxed_sql()).collect::<Vec<BoxedSql>>();
                boxed.modify(|v| format!("{v} IN ({})", other_boxed.iter().map(|i| &*i.sql).collect::<Vec<&str>>().join(",")));
                for b in other_boxed {
                    boxed.values.extend(b.values);
                }

                QueryCondition::new(boxed)
            }

            fn not_in_vec(
                &self,
                other: Vec<impl IntoSql<$column_type>>,
            ) -> QueryCondition<U> {
                let mut boxed = self.get_sql();
                let other_boxed = other.iter().map(|i| i.into_boxed_sql()).collect::<Vec<BoxedSql>>();
                boxed.modify(|v| format!("{v} NOT IN ({})", other_boxed.iter().map(|i| &*i.sql).collect::<Vec<&str>>().join(",")));
                for b in other_boxed {
                    boxed.values.extend(b.values);
                }

                QueryCondition::new(boxed)
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
