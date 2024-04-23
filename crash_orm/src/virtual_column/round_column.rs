use tokio_postgres::types::ToSql;

use crate::{BoxedColumnValue, Column, Entity, PrimaryKey, VirtualColumn};

pub trait RoundVirtualColumn<T: ToSql, R: ToSql, U: Entity<U, P>, P: PrimaryKey> {
    fn ceil(&self) -> VirtualColumn<R, U, P>;

    fn floor(&self) -> VirtualColumn<R, U, P>;

    fn round(&self) -> VirtualColumn<R, U, P>;
}

macro_rules! impl_round_virtual_column {
    ($column_type:ty, $out_type:ty) => {
        impl<U: Entity<U, P>, R: Column<$column_type, U, P>, P: PrimaryKey>
            RoundVirtualColumn<$column_type, $out_type, U, P> for R
        {
            fn ceil(&self) -> VirtualColumn<$out_type, U, P> {
                let sql = self.get_sql();
                VirtualColumn::new(BoxedColumnValue::new(
                    format!("CEIL({})", sql.sql),
                    sql.value,
                ))
            }

            fn floor(&self) -> VirtualColumn<$out_type, U, P> {
                let sql = self.get_sql();
                VirtualColumn::new(BoxedColumnValue::new(
                    format!("FLOOR({})", sql.sql),
                    sql.value,
                ))
            }

            fn round(&self) -> VirtualColumn<$out_type, U, P> {
                let sql = self.get_sql();
                VirtualColumn::new(BoxedColumnValue::new(
                    format!("ROUND({})", sql.sql),
                    sql.value,
                ))
            }
        }
    };
}

impl_round_virtual_column!(f32, f64);
impl_round_virtual_column!(f64, f64);
#[cfg(feature = "with-rust-decimal")]
impl_round_virtual_column!(rust_decimal::Decimal, rust_decimal::Decimal);
