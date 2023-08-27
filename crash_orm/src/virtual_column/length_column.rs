use crate::{Entity, EntityColumn};

pub trait LengthVirtualColumn<U: Entity<U> + Send + 'static> {
    fn length(&self) -> EntityColumn<i32, U>;
}

impl<U: Entity<U> + Send + 'static> LengthVirtualColumn<U> for EntityColumn<'_, String, U> {
    fn length(&self) -> EntityColumn<i32, U> {
        EntityColumn::from_string(format!("LENGTH({})", self.get_name()))
    }
}

impl<U: Entity<U> + Send + 'static> LengthVirtualColumn<U> for EntityColumn<'_, Option<String>, U> {
    fn length(&self) -> EntityColumn<i32, U> {
        EntityColumn::from_string(format!("LENGTH({})", self.get_name()))
    }
}