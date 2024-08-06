use tokio_postgres::types::ToSql;

use crate::prelude::{BoxedColumnValue, Column, Entity, PrimaryKey, VirtualColumn};

/// Trait implementing cast to text database functions to create [VirtualColumn]s for all sort of columns
pub trait TextCastVirtualColumn<T: ToSql, U: Entity<U, P>, P: PrimaryKey> {
    /// Cast self to text (string)
    fn cast_to_text(&self) -> VirtualColumn<String, U, P>;
}

macro_rules! impl_text_cast_virtual_column {
    ($column_type:ty) => {
        impl<U: Entity<U, P>, R: Column<$column_type, U, P>, P: PrimaryKey> TextCastVirtualColumn<$column_type, U, P>
            for R
        {
            fn cast_to_text(&self) -> VirtualColumn<String, U, P> {
                let sql = self.get_sql();
                VirtualColumn::new(BoxedColumnValue::new(
                    format!("CAST({} AS TEXT)", sql.sql),
                    sql.value,
                ))
            }
        }
    };
}

impl_text_cast_virtual_column!(bool);
impl_text_cast_virtual_column!(i8);
impl_text_cast_virtual_column!(i16);
impl_text_cast_virtual_column!(i32);
impl_text_cast_virtual_column!(i64);
#[cfg(feature = "with-rust-decimal")]
impl_text_cast_virtual_column!(rust_decimal::Decimal);
impl_text_cast_virtual_column!(u32);
impl_text_cast_virtual_column!(f32);
impl_text_cast_virtual_column!(f64);
#[cfg(feature = "with-chrono")]
impl_text_cast_virtual_column!(chrono::NaiveDateTime);
#[cfg(feature = "with-chrono")]
impl_text_cast_virtual_column!(chrono::DateTime<chrono::Utc>);
#[cfg(feature = "with-chrono")]
impl_text_cast_virtual_column!(chrono::DateTime<chrono::Local>);
#[cfg(feature = "with-chrono")]
impl_text_cast_virtual_column!(chrono::DateTime<chrono::FixedOffset>);
#[cfg(feature = "with-chrono")]
impl_text_cast_virtual_column!(chrono::NaiveDate);
#[cfg(feature = "with-chrono")]
impl_text_cast_virtual_column!(chrono::NaiveTime);
