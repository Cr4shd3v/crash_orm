use crate::{BoxedColumnValue, Column, Entity, PrimaryKey, TypedColumnValue, UntypedColumnValue, VirtualColumn};

pub trait StringVirtualColumn<U: Entity<U, PRIMARY>, PRIMARY: PrimaryKey<'static>> {
    fn lowercase(&self) -> VirtualColumn<String, U, PRIMARY>;

    fn uppercase(&self) -> VirtualColumn<String, U, PRIMARY>;

    fn reverse(&self) -> VirtualColumn<String, U, PRIMARY>;

    fn length(&self) -> VirtualColumn<i32, U, PRIMARY>;

    fn repeat(&self, repetition: &(dyn TypedColumnValue<i32>)) -> VirtualColumn<String, U, PRIMARY>;

    fn concat(&self, other: Vec<&(dyn UntypedColumnValue)>) -> VirtualColumn<String, U, PRIMARY>;

    fn md5(&self) -> VirtualColumn<String, U, PRIMARY>;
}

impl<U: Entity<U, PRIMARY>, R: Column<String, U, PRIMARY>, PRIMARY: PrimaryKey<'static>> StringVirtualColumn<U, PRIMARY> for R {
    fn lowercase(&self) -> VirtualColumn<String, U, PRIMARY> {
        let sql = self.get_sql();
        VirtualColumn::new(BoxedColumnValue::new(
            format!("LOWER({})", sql.sql),
            sql.value,
        ))
    }

    fn uppercase(&self) -> VirtualColumn<String, U, PRIMARY> {
        let sql = self.get_sql();
        VirtualColumn::new(BoxedColumnValue::new(
            format!("UPPER({})", sql.sql),
            sql.value,
        ))
    }

    fn reverse(&self) -> VirtualColumn<String, U, PRIMARY> {
        let sql = self.get_sql();
        VirtualColumn::new(BoxedColumnValue::new(
            format!("REVERSE({})", sql.sql),
            sql.value,
        ))
    }

    fn length(&self) -> VirtualColumn<i32, U, PRIMARY> {
        let sql = self.get_sql();
        VirtualColumn::new(BoxedColumnValue::new(
            format!("LENGTH({})", sql.sql),
            sql.value,
        ))
    }

    fn repeat(&self, repetition: &(dyn TypedColumnValue<i32>)) -> VirtualColumn<String, U, PRIMARY> {
        let sql = self.get_sql();
        let repetition_sql = repetition.get_sql();
        let mut values = sql.value;
        values.extend(repetition_sql.value);
        VirtualColumn::new(BoxedColumnValue::new(
            format!("REPEAT({},{})", sql.sql, repetition_sql.sql),
            values,
        ))
    }

    fn concat(&self, other: Vec<&(dyn UntypedColumnValue)>) -> VirtualColumn<String, U, PRIMARY> {
        let mut sql = self.get_sql();
        for value in other {
            let value_sql = value.get_sql();
            sql.sql.push_str(" || ");
            sql.sql.push_str(&*value_sql.sql);
            sql.value.extend(value_sql.value);
        }

        VirtualColumn::new(sql)
    }

    fn md5(&self) -> VirtualColumn<String, U, PRIMARY> {
        let sql = self.get_sql();
        VirtualColumn::new(BoxedColumnValue::new(
            format!("MD5({})", sql.sql),
            sql.value,
        ))
    }
}
