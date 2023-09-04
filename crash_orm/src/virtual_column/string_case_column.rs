use crate::{BoxedColumnValue, Column, Entity, VirtualColumn};

pub trait StringCaseVirtualColumn<U: Entity<U>> {
    fn lowercase(&self) -> VirtualColumn<String, U>;

    fn uppercase(&self) -> VirtualColumn<String, U>;
}

impl<U: Entity<U>, R: Column<String, U>> StringCaseVirtualColumn<U> for R {
    fn lowercase(&self) -> VirtualColumn<String, U> {
        let sql = self.get_sql();
        VirtualColumn::new(BoxedColumnValue::new(format!("LOWER({})", sql.sql), sql.value))
    }

    fn uppercase(&self) -> VirtualColumn<String, U> {
        let sql = self.get_sql();
        VirtualColumn::new(BoxedColumnValue::new(format!("UPPER({})", sql.sql), sql.value))
    }
}