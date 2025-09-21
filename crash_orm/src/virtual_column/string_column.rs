use crate::prelude::{BoxedSql, Column, Entity, IntoSql, UntypedColumnValue, VirtualColumn};

/// Trait implementing string database functions to create [VirtualColumn]s for string columns
pub trait StringVirtualColumn<U: Entity> {
    /// Convert self to lowercase
    fn lowercase(&self) -> VirtualColumn<String, U>;

    /// Convert self to uppercase
    fn uppercase(&self) -> VirtualColumn<String, U>;

    /// Reverse self
    fn reverse(&self) -> VirtualColumn<String, U>;

    /// Get the length of self
    fn length(&self) -> VirtualColumn<i32, U>;

    /// Repeat self `repetition` times
    fn repeat(&self, repetition: impl IntoSql<i32>) -> VirtualColumn<String, U>;

    /// Concat self and other
    fn concat(&self, other: Vec<&dyn UntypedColumnValue>) -> VirtualColumn<String, U>;

    /// Creates the md5 hash of this string
    fn md5(&self) -> VirtualColumn<String, U>;
}

impl<U: Entity, R: Column<String, U>> StringVirtualColumn<U> for R {
    fn lowercase(&self) -> VirtualColumn<String, U> {
        let sql = self.get_sql();
        VirtualColumn::new(BoxedSql::new(
            format!("LOWER({})", sql.sql),
            sql.values,
        ))
    }

    fn uppercase(&self) -> VirtualColumn<String, U> {
        let sql = self.get_sql();
        VirtualColumn::new(BoxedSql::new(
            format!("UPPER({})", sql.sql),
            sql.values,
        ))
    }

    fn reverse(&self) -> VirtualColumn<String, U> {
        let sql = self.get_sql();
        VirtualColumn::new(BoxedSql::new(
            format!("REVERSE({})", sql.sql),
            sql.values,
        ))
    }

    fn length(&self) -> VirtualColumn<i32, U> {
        let sql = self.get_sql();
        VirtualColumn::new(BoxedSql::new(
            format!("LENGTH({})", sql.sql),
            sql.values,
        ))
    }

    fn repeat(&self, repetition: impl IntoSql<i32>) -> VirtualColumn<String, U> {
        let sql = self.get_sql();
        let repetition_sql = repetition.into_boxed_sql();
        let mut values = sql.values;
        values.extend(repetition_sql.values);
        VirtualColumn::new(BoxedSql::new(
            format!("REPEAT({},{})", sql.sql, repetition_sql.sql),
            values,
        ))
    }

    fn concat(&self, other: Vec<&dyn UntypedColumnValue>) -> VirtualColumn<String, U> {
        let mut sql = self.get_sql();
        for value in other {
            let value_sql = value.get_sql();
            sql.sql.push_str(" || ");
            sql.sql.push_str(&*value_sql.sql);
            sql.values.extend(value_sql.values);
        }

        VirtualColumn::new(sql)
    }

    fn md5(&self) -> VirtualColumn<String, U> {
        let sql = self.get_sql();
        VirtualColumn::new(BoxedSql::new(
            format!("MD5({})", sql.sql),
            sql.values,
        ))
    }
}
