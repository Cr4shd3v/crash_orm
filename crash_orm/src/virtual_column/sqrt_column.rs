use crate::{BoxedColumnValue, Column, Entity, PrimaryKey, VirtualColumn};
use tokio_postgres::types::ToSql;

pub trait SqrtVirtualColumn<T: ToSql, U: Entity<U, PRIMARY>, PRIMARY: PrimaryKey<'static>> {
    fn sqrt(&self) -> VirtualColumn<T, U, PRIMARY>;
}

macro_rules! impl_sqrt_virtual_column {
    ($column_type:ty) => {
        impl<U: Entity<U, PRIMARY>, R: Column<$column_type, U, PRIMARY>, PRIMARY: PrimaryKey<'static>> SqrtVirtualColumn<$column_type, U, PRIMARY> for R {
            fn sqrt(&self) -> VirtualColumn<$column_type, U, PRIMARY> {
                let sql = self.get_sql();
                VirtualColumn::new(BoxedColumnValue::new(
                    format!("SQRT({})", sql.sql),
                    sql.value,
                ))
            }
        }
    };
}

impl_sqrt_virtual_column!(f64);
#[cfg(feature = "with-rust-decimal")]
impl_sqrt_virtual_column!(rust_decimal::Decimal);
