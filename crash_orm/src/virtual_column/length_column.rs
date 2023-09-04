use crate::{BoxedColumnValue, Column, Entity, VirtualColumn};

pub trait LengthVirtualColumn<U: Entity<U>> {
    fn length(&self) -> VirtualColumn<i32, U>;
}

impl<U: Entity<U>, R: Column<String, U>> LengthVirtualColumn<U> for R {
    fn length(&self) -> VirtualColumn<i32, U> {
        let sql = self.get_sql();
        VirtualColumn::new(BoxedColumnValue::new(format!("LENGTH({})", sql.sql), sql.value))
    }
}