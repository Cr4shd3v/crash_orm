use rust_decimal::Decimal;
use tokio_postgres::types::ToSql;
use crate::{Column, Entity, VirtualColumn};

pub trait RoundVirtualColumn<T: ToSql, R: ToSql, U: Entity<U> + Send + 'static> {
    fn ceil(&self) -> VirtualColumn<R, U>;

    fn floor(&self) -> VirtualColumn<R, U>;

    fn round(&self) -> VirtualColumn<R, U>;
}

macro_rules! impl_round_virtual_column {
    ($column_type:ty, $out_type:ty) => {
        impl<U: Entity<U> + Send + 'static, R: Column<$column_type, U>> RoundVirtualColumn<$column_type, $out_type, U> for R {
            fn ceil(&self) -> VirtualColumn<$out_type, U> {
                VirtualColumn::new(format!("CEIL({})", self.get_name()))
            }

            fn floor(&self) -> VirtualColumn<$out_type, U> {
                VirtualColumn::new(format!("FLOOR({})", self.get_name()))
            }

            fn round(&self) -> VirtualColumn<$out_type, U> {
                VirtualColumn::new(format!("ROUND({})", self.get_name()))
            }
        }
    };
}

impl_round_virtual_column!(f32, f64);
impl_round_virtual_column!(f64, f64);
impl_round_virtual_column!(Decimal, Decimal);