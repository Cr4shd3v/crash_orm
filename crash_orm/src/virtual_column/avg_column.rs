use crate::prelude::{BoxedSql, Column, ColumnType, Entity, VirtualColumn};

/// Trait implementing the count function for a column.
///
/// Note, that aggregated values like this cannot be used in where statements.
pub trait AvgColumn<T: ColumnType, R: ColumnType, U: Entity> {
    /// Avg function
    fn avg(&self, distinct: bool) -> VirtualColumn<R, U>;
}

macro_rules! impl_avg_column {
    ($in_type:ty, $out_type:ty) => {
        impl<U: Entity, C: Column<$in_type, U>> AvgColumn<$in_type, $out_type, U> for C {
            fn avg(&self, distinct: bool) -> VirtualColumn<$out_type, U> {
                let sql = self.get_sql();
                VirtualColumn::new(BoxedSql::new(
                    format!("AVG({}{})", if distinct { "DISTINCT " } else { "" }, sql.sql),
                    sql.values,
                ))
            }
        }
    }
}

#[cfg(feature = "with-rust-decimal")]
impl_avg_column!(i8, rust_decimal::Decimal);
#[cfg(feature = "with-rust-decimal")]
impl_avg_column!(i16, rust_decimal::Decimal);
#[cfg(feature = "with-rust-decimal")]
impl_avg_column!(i32, rust_decimal::Decimal);
#[cfg(feature = "with-rust-decimal")]
impl_avg_column!(i64, rust_decimal::Decimal);
#[cfg(feature = "with-rust-decimal")]
impl_avg_column!(rust_decimal::Decimal, rust_decimal::Decimal);
impl_avg_column!(f32, f64);
impl_avg_column!(f64, f64);