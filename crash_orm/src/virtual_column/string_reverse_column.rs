use crate::{BoxedColumnValue, Column, Entity, VirtualColumn};

pub trait StringReverseVirtualColumn<U: Entity<U>> {
    fn reverse(&self) -> VirtualColumn<String, U>;
}

impl<U: Entity<U>, R: Column<String, U>> StringReverseVirtualColumn<U> for R {
    fn reverse(&self) -> VirtualColumn<String, U> {
        let sql = self.get_sql();
        VirtualColumn::new(BoxedColumnValue::new(format!("REVERSE({})", sql.sql), sql.value))
    }
}