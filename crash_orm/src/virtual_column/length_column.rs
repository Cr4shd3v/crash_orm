use crate::{Column, Entity, VirtualColumn};

pub trait LengthVirtualColumn<U: Entity<U> + Send + 'static> {
    fn length(&self) -> VirtualColumn<i32, U>;
}

impl<U: Entity<U> + Send + 'static, R: Column<String, U>> LengthVirtualColumn<U> for R {
    fn length(&self) -> VirtualColumn<i32, U> {
        VirtualColumn::new(format!("LENGTH({})", self.get_name()))
    }
}