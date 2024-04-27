use std::marker::PhantomData;

use crate::{Entity, PrimaryKey};
use crate::relations::macros::{default_relation_function, sql_impl_for_relation};

/// Struct representing the owning site of a 1:1 relationship.
///
/// This actually holds the value of the relationship compared to [OneToOneRef].
#[derive(Debug)]
pub struct OneToOne<T: Entity<T, P>, P: PrimaryKey> {
    _p: PhantomData<T>,
    /// Raw id of the relation
    pub target_id: P,
}

impl<T: Entity<T, P>, P: PrimaryKey> OneToOne<T, P> {
    default_relation_function!(OneToOne);
}

sql_impl_for_relation!(OneToOne);

/// Struct representing the unowned site of the 1:1 relationship.
///
/// For the owning site of this relation, see [crate::OneToOne].
///
/// Requires the mapped_by attribute to work as shown below.
/// ```
/// use crash_orm::{OneToOne, OneToOneRef, Entity};
/// use crash_orm_derive::Entity;
///
/// #[derive(Entity, Debug)]
/// struct TestItem1 {
///     id: Option<u32>,
///     item2: OneToOne<TestItem2, u32>,
/// }
///
/// #[derive(Entity, Debug)]
/// struct TestItem2 {
///     id: Option<u32>,
///     test: String,
///     #[mapped_by("item2")]
///     item1: OneToOneRef<TestItem1, u32>,
/// }
/// ```
#[derive(Debug)]
pub struct OneToOneRef<T: Entity<T, P>, P: PrimaryKey> {
    _p: PhantomData<T>,
    _p1: PhantomData<P>,
}

impl<T: Entity<T, P>, P: PrimaryKey> OneToOneRef<T, P> {
    /// Constructs the unowned site of the 1:1 relation
    pub fn new() -> OneToOneRef<T, P> {
        OneToOneRef { _p: PhantomData, _p1: PhantomData }
    }
}