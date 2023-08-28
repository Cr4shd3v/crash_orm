use crate::{Column, Entity, VirtualColumn};

pub trait StringCaseVirtualColumn<U: Entity<U> + Send + 'static> {
    fn lowercase(&self) -> VirtualColumn<String, U>;

    fn uppercase(&self) -> VirtualColumn<String, U>;
}

impl<U: Entity<U> + Send + 'static, R: Column<String, U>> StringCaseVirtualColumn<U> for R {
    fn lowercase(&self) -> VirtualColumn<String, U> {
        VirtualColumn::new(format!("LOWER({})", self.get_name()))
    }

    fn uppercase(&self) -> VirtualColumn<String, U> {
        VirtualColumn::new(format!("UPPER({})", self.get_name()))
    }
}