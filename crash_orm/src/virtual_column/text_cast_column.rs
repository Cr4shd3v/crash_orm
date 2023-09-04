use rust_decimal::Decimal;
use tokio_postgres::types::ToSql;
use crate::{BoxedColumnValue, Column, Entity, VirtualColumn};

pub trait TextCastVirtualColumn<T: ToSql, U: Entity<U>> {
    fn cast_to_text(&self) -> VirtualColumn<String, U>;
}

macro_rules! impl_text_cast_virtual_column {
    ($column_type:ty) => {
        impl<U: Entity<U>, R: Column<$column_type, U>> TextCastVirtualColumn<$column_type, U> for R {
            fn cast_to_text(&self) -> VirtualColumn<String, U> {
                let sql = self.get_sql();
                VirtualColumn::new(BoxedColumnValue::new(format!("CAST({} AS TEXT)", sql.sql), sql.value))
            }
        }
    };
}

impl_text_cast_virtual_column!(i8);
impl_text_cast_virtual_column!(i16);
impl_text_cast_virtual_column!(i32);
impl_text_cast_virtual_column!(i64);
impl_text_cast_virtual_column!(Decimal);
impl_text_cast_virtual_column!(f32);
impl_text_cast_virtual_column!(f64);