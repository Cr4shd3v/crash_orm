use crate::prelude::{BoxedSql, Column, ColumnType, Entity, VirtualColumn};

/// Trait implementing cast to text database functions to create [VirtualColumn]s for all sort of columns
pub trait TextCastVirtualColumn<T: ColumnType, U: Entity> {
    /// Cast self to text (string)
    fn cast_to_text(&self) -> VirtualColumn<String, U>;
}

macro_rules! impl_text_cast_virtual_column {
    ($column_type:ty) => {
        impl<U: Entity, R: Column<$column_type, U>> TextCastVirtualColumn<$column_type, U>
            for R
        {
            fn cast_to_text(&self) -> VirtualColumn<String, U> {
                let sql = self.get_sql();
                VirtualColumn::new(BoxedSql::new(
                    format!("CAST({} AS TEXT)", sql.sql),
                    sql.values,
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
