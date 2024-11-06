use crate::prelude::{BoxedSql, Column, Entity, VirtualColumn};
use postgres::types::ToSql;

/// Trait implementing the sum function for a column.
///
/// Note, that aggregated values like this cannot be used in where statements.
pub trait SumColumn<T: ToSql, R: ToSql, U: Entity> {
    /// Sum function
    fn sum(&self, distinct: bool) -> VirtualColumn<R, U>;
}


macro_rules! impl_sum_column {
    ($in_type:ty, $out_type:ty) => {
        impl<U: Entity, C: Column<$in_type, U>> SumColumn<$in_type, $out_type, U> for C {
            fn sum(&self, distinct: bool) -> VirtualColumn<$out_type, U> {
                let sql = self.get_sql();
                VirtualColumn::new(BoxedSql::new(
                    format!("SUM({}{})", if distinct { "DISTINCT " } else { "" }, sql.sql),
                    sql.values,
                ))
            }
        }
    }
}

impl_sum_column!(i8, i64);
impl_sum_column!(i16, i64);
impl_sum_column!(i32, i64);
#[cfg(feature = "with-rust-decimal")]
impl_sum_column!(i64, rust_decimal::Decimal);
#[cfg(feature = "with-rust-decimal")]
impl_sum_column!(rust_decimal::Decimal, rust_decimal::Decimal);
impl_sum_column!(f32, f64);
impl_sum_column!(f64, f64);

