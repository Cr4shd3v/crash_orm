use tokio_postgres::types::ToSql;

use crate::prelude::{BoxedSql, Column, Entity, VirtualColumn};

/// Trait implementing sqrt database function to create [VirtualColumn] for decimal columns
pub trait SqrtVirtualColumn<T: ToSql, U: Entity> {
    /// Get square root of self
    fn sqrt(&self) -> VirtualColumn<T, U>;
}

macro_rules! impl_sqrt_virtual_column {
    ($column_type:ty) => {
        impl<U: Entity, R: Column<$column_type, U>> SqrtVirtualColumn<$column_type, U> for R {
            fn sqrt(&self) -> VirtualColumn<$column_type, U> {
                let sql = self.get_sql();
                VirtualColumn::new(BoxedSql::new(
                    format!("SQRT({})", sql.sql),
                    sql.values,
                ))
            }
        }
    };
}

impl_sqrt_virtual_column!(f64);
#[cfg(feature = "with-rust-decimal")]
impl_sqrt_virtual_column!(rust_decimal::Decimal);
