use crate::entity::Entity;
use crate::prelude::{BoxedSql, Column, VirtualColumn};
use postgres::types::ToSql;

/// Trait implementing the max function for a column.
///
/// Note, that aggregated values like this cannot be used in where statements.
pub trait MaxColumn<T: ToSql, U: Entity> {
    /// Max function
    fn max(&self) -> VirtualColumn<T, U>;
}


macro_rules! impl_min_column {
    ($in_type:ty) => {
        impl<U: Entity, C: Column<$in_type, U>> MaxColumn<$in_type, U> for C {
            fn max(&self) -> VirtualColumn<$in_type, U> {
                let sql = self.get_sql();
                VirtualColumn::new(BoxedSql::new(
                    format!("MAX({})", sql.sql),
                    sql.values,
                ))
            }
        }
    }
}

impl_min_column!(i8);
impl_min_column!(i16);
impl_min_column!(i32);
impl_min_column!(i64);
#[cfg(feature = "with-rust-decimal")]
impl_min_column!(rust_decimal::Decimal);
impl_min_column!(u32);
impl_min_column!(f32);
impl_min_column!(f64);
impl_min_column!(String);
