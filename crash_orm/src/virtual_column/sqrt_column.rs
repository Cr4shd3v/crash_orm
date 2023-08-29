use rust_decimal::Decimal;
use tokio_postgres::types::ToSql;
use crate::{Column, Entity, VirtualColumn};

pub trait SqrtVirtualColumn<T: ToSql, U: Entity<U>> {
    fn sqrt(&self) -> VirtualColumn<T, U>;
}

macro_rules! impl_sqrt_virtual_column {
    ($column_type:ty) => {
        impl<U: Entity<U>, R: Column<$column_type, U>> SqrtVirtualColumn<$column_type, U> for R {
            fn sqrt(&self) -> VirtualColumn<$column_type, U> {
                VirtualColumn::new(format!("SQRT({})", self.get_sql()))
            }
        }
    };
}

impl_sqrt_virtual_column!(f64);
impl_sqrt_virtual_column!(Decimal);