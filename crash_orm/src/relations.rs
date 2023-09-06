use crate::{DatabaseConnection, Entity};

pub struct OneToOneOwner<T: Entity<T>> {
    target_id: u32,
    value: Option<T>,
}

impl<T: Entity<T>> OneToOneOwner<T> {
    pub const fn new(target_id: u32) -> OneToOneOwner<T> {
        Self {
            target_id,
            value: None,
        }
    }

    pub async fn get(&mut self, conn: &DatabaseConnection) -> crate::Result<&T> {
        if self.value.is_none() {
            self.value = Some(T::get_by_id(&conn, self.target_id).await?);
        }

        Ok(self.value.as_ref().unwrap())
    }
}

pub struct OneToOne<T: Entity<T>> {
    value: Option<T>,
}

impl<T: Entity<T>> OneToOne<T> {
    pub const fn new() -> OneToOne<T> {
        Self {
            value: None,
        }
    }
}

pub struct OneToMany<T: Entity<T>> {
    value: Option<Vec<T>>,
}

impl<T: Entity<T>> OneToMany<T> {
    pub const fn new() -> OneToMany<T> {
        Self {
            value: None,
        }
    }
}

pub struct ManyToOne<T: Entity<T>> {
    target_id: u32,
    value: Option<T>,
}

impl<T: Entity<T>> ManyToOne<T> {
    pub const fn new(target_id: u32) -> ManyToOne<T> {
        Self {
            target_id,
            value: None,
        }
    }

    pub async fn get(&mut self, conn: &DatabaseConnection) -> crate::Result<&T> {
        if self.value.is_none() {
            self.value = Some(T::get_by_id(&conn, self.target_id).await?);
        }

        Ok(self.value.as_ref().unwrap())
    }
}

pub struct ManyToMany<T: Entity<T>> {
    value: Option<Vec<T>>,
}

impl<T: Entity<T>> ManyToMany<T> {
    pub const fn new() -> ManyToMany<T> {
        Self {
            value: None,
        }
    }
}