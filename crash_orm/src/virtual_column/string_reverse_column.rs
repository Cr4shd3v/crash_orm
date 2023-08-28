use crate::{Column, Entity, VirtualColumn};

pub trait StringReverseVirtualColumn<U: Entity<U>> {
    fn reverse(&self) -> VirtualColumn<String, U>;
}

impl<U: Entity<U>, R: Column<String, U>> StringReverseVirtualColumn<U> for R {
    fn reverse(&self) -> VirtualColumn<String, U> {
        VirtualColumn::new(format!("REVERSE({})", self.get_name()))
    }
}