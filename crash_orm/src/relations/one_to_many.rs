use std::marker::PhantomData;

use crate::prelude::{ColumnType, PrimaryKeyEntity};
use crate::relations::macros::{default_relation_function, sql_impl_for_relation};

/// Struct representing the many site of the n:1 relationship.
///
/// This struct holds the value of the relation.
///
/// The counterpart for [ManyToOne] is [OneToMany].
#[derive(Debug)]
pub struct ManyToOne<T: PrimaryKeyEntity<P>, P: ColumnType> {
    _p: PhantomData<T>,
    /// Raw id of the relation
    pub target_id: P,
}

impl<T: PrimaryKeyEntity<P>, P: ColumnType> ManyToOne<T, P> {
    default_relation_function!(ManyToOne);
}

sql_impl_for_relation!(ManyToOne);

/// Struct representing the one site of the n:1 relationship.
///
/// The counterpart for [OneToMany] is [ManyToOne].
///
/// Requires the mapped_by attribute to work as shown below.
/// ```
/// use crash_orm::prelude::*;
///
/// #[derive(Entity, Debug)]
/// struct TestItem1 {
///     id: u32,
///     item2: ManyToOne<TestItem2, u32>,
/// }
///
/// #[derive(Entity, Debug)]
/// struct TestItem2 {
///     id: u32,
///     test: String,
///     #[mapped_by("item2")]
///     item1: OneToMany<TestItem1, u32>,
/// }
/// ```
#[derive(Debug)]
pub struct OneToMany<T: PrimaryKeyEntity<P>, P: ColumnType> {
    _p: PhantomData<T>,
    _p1: PhantomData<P>,
}

impl<T: PrimaryKeyEntity<P>, P: ColumnType> OneToMany<T, P> {
    /// Constructs a 1:n relation
    pub fn new() -> OneToMany<T, P> {
        OneToMany { _p: PhantomData, _p1: PhantomData }
    }
}
