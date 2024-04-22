use crate::{BoxedColumnValue, Column, Entity, PrimaryKey, VirtualColumn};
use tokio_postgres::types::ToSql;

pub trait RoundVirtualColumn<T: ToSql, R: ToSql, U: Entity<U, PRIMARY>, PRIMARY: PrimaryKey<'static>> {
    fn ceil(&self) -> VirtualColumn<R, U, PRIMARY>;

    fn floor(&self) -> VirtualColumn<R, U, PRIMARY>;

    fn round(&self) -> VirtualColumn<R, U, PRIMARY>;
}

macro_rules! impl_round_virtual_column {
    ($column_type:ty, $out_type:ty) => {
        impl<U: Entity<U, PRIMARY>, R: Column<$column_type, U, PRIMARY>, PRIMARY: PrimaryKey<'static>>
            RoundVirtualColumn<$column_type, $out_type, U, PRIMARY> for R
        {
            fn ceil(&self) -> VirtualColumn<$out_type, U, PRIMARY> {
                let sql = self.get_sql();
                VirtualColumn::new(BoxedColumnValue::new(
                    format!("CEIL({})", sql.sql),
                    sql.value,
                ))
            }

            fn floor(&self) -> VirtualColumn<$out_type, U, PRIMARY> {
                let sql = self.get_sql();
                VirtualColumn::new(BoxedColumnValue::new(
                    format!("FLOOR({})", sql.sql),
                    sql.value,
                ))
            }

            fn round(&self) -> VirtualColumn<$out_type, U, PRIMARY> {
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
