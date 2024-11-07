use crate::prelude::{BoxedSql, Column, ColumnType, Entity, VirtualColumn};

/// Trait implementing round database functions to create [VirtualColumn]s for number columns
pub trait RoundVirtualColumn<T: ColumnType, R: ColumnType, U: Entity> {
    /// Ceil function
    fn ceil(&self) -> VirtualColumn<R, U>;

    /// Floor function
    fn floor(&self) -> VirtualColumn<R, U>;

    /// Round function
    fn round(&self) -> VirtualColumn<R, U>;
}

macro_rules! impl_round_virtual_column {
    ($column_type:ty, $out_type:ty) => {
        impl<U: Entity, R: Column<$column_type, U>>
            RoundVirtualColumn<$column_type, $out_type, U> for R
        {
            fn ceil(&self) -> VirtualColumn<$out_type, U> {
                let sql = self.get_sql();
                VirtualColumn::new(BoxedSql::new(
                    format!("CEIL({})", sql.sql),
                    sql.values,
                ))
            }

            fn floor(&self) -> VirtualColumn<$out_type, U> {
                let sql = self.get_sql();
                VirtualColumn::new(BoxedSql::new(
                    format!("FLOOR({})", sql.sql),
                    sql.values,
                ))
            }

            fn round(&self) -> VirtualColumn<$out_type, U> {
                let sql = self.get_sql();
                VirtualColumn::new(BoxedSql::new(
                    format!("ROUND({})", sql.sql),
                    sql.values,
                ))
            }
        }
    };
}

impl_round_virtual_column!(f32, f64);
impl_round_virtual_column!(f64, f64);
#[cfg(feature = "with-rust-decimal")]
impl_round_virtual_column!(rust_decimal::Decimal, rust_decimal::Decimal);
