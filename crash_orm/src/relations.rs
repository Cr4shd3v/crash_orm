use crate::Entity;

pub struct OneToOneOwner<T: Entity<T>> {
    value: Option<T>,
}

pub struct OneToOne<T: Entity<T>> {
    value: Option<T>,
}

pub struct OneToMany<T: Entity<T>> {
    value: Option<Vec<T>>,
}

pub struct ManyToOne<T: Entity<T>> {
    value: Option<T>,
}

pub struct ManyToMany<T: Entity<T>> {
    value: Option<Vec<T>>,
}