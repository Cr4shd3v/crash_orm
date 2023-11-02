use crate::{BoxedColumnValue, Column, Entity, BoundColumnValue, UnboundColumnValue, VirtualColumn};

pub trait StringVirtualColumn<U: Entity<U>> {
    fn lowercase(&self) -> VirtualColumn<String, U>;

    fn uppercase(&self) -> VirtualColumn<String, U>;

    fn reverse(&self) -> VirtualColumn<String, U>;

    fn length(&self) -> VirtualColumn<i32, U>;

    fn repeat(&self, repetition: &(dyn BoundColumnValue<i32>)) -> VirtualColumn<String, U>;

    fn concat(&self, other: Vec<&(dyn UnboundColumnValue)>) -> VirtualColumn<String, U>;

    fn md5(&self) -> VirtualColumn<String, U>;
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

    fn repeat(&self, repetition: &(dyn BoundColumnValue<i32>)) -> VirtualColumn<String, U> {
        let sql = self.get_sql();
        let repetition_sql = repetition.get_sql();
        let mut values = sql.value;
        values.extend(repetition_sql.value);
        VirtualColumn::new(BoxedColumnValue::new(format!("REPEAT({},{})", sql.sql, repetition_sql.sql), values))
    }

    fn concat(&self, other: Vec<&(dyn UnboundColumnValue)>) -> VirtualColumn<String, U> {
        let mut sql = self.get_sql();
        for value in other {
            let value_sql = value.get_sql();
            sql.sql.push_str(" || ");
            sql.sql.push_str(&*value_sql.sql);
            sql.value.extend(value_sql.value);
        }

        VirtualColumn::new(sql)
    }

    fn md5(&self) -> VirtualColumn<String, U> {
        let sql = self.get_sql();
        VirtualColumn::new(BoxedColumnValue::new(format!("MD5({})", sql.sql), sql.value))
    }
}