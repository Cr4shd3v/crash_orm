use crate::{BoxedColumnValue, Column, Entity, TypedColumnValue, VirtualColumn};

pub trait StringVirtualColumn<U: Entity<U>> {
    fn lowercase(&self) -> VirtualColumn<String, U>;

    fn uppercase(&self) -> VirtualColumn<String, U>;

    fn reverse(&self) -> VirtualColumn<String, U>;

    fn length(&self) -> VirtualColumn<i32, U>;

    fn repeat(&self, repetition: &(dyn TypedColumnValue<i32>)) -> VirtualColumn<String, U>;
}

impl<U: Entity<U>, R: Column<String, U>> StringVirtualColumn<U> for R {
    fn lowercase(&self) -> VirtualColumn<String, U> {
        let sql = self.get_sql();
        VirtualColumn::new(BoxedColumnValue::new(format!("LOWER({})", sql.sql), sql.value))
    }

    fn uppercase(&self) -> VirtualColumn<String, U> {
        let sql = self.get_sql();
        VirtualColumn::new(BoxedColumnValue::new(format!("UPPER({})", sql.sql), sql.value))
    }

    fn reverse(&self) -> VirtualColumn<String, U> {
        let sql = self.get_sql();
        VirtualColumn::new(BoxedColumnValue::new(format!("REVERSE({})", sql.sql), sql.value))
    }

    fn length(&self) -> VirtualColumn<i32, U> {
        let sql = self.get_sql();
        VirtualColumn::new(BoxedColumnValue::new(format!("LENGTH({})", sql.sql), sql.value))
    }

    fn repeat(&self, repetition: &(dyn TypedColumnValue<i32>)) -> VirtualColumn<String, U> {
        let sql = self.get_sql();
        let repetition_sql = repetition.get_sql();
        let mut values = sql.value;
        values.extend(repetition_sql.value);
        VirtualColumn::new(BoxedColumnValue::new(format!("REPEAT({},{})", sql.sql, repetition_sql.sql), values))
    }
}