use std::error::Error;
use std::fmt::Debug;
use std::marker::PhantomData;

use tokio_postgres::types::{FromSql, IsNull, ToSql, Type};
use tokio_postgres::types::private::BytesMut;

use crate::{Entity, PrimaryKey};

macro_rules! default_relation_function {
    ($rel_type:tt) => {
        /// Creates the relation with a given id.
        pub const fn new(target_id: P) -> $rel_type<T, P> {
            Self {
                _p: PhantomData,
                target_id,
            }
        }

        /// Creates the relation from an entity.
        ///
        /// This utilizes the [Entity::get_id] function.
        pub fn from(entity: &impl Entity<T, P>) -> crate::Result<$rel_type<T, P>> {
            let id = entity.get_id();
            if id.is_none() {
                return Err(crate::Error::from_str(
                    "Can't link an entity that hasn't been inserted yet",
                ));
            }

            Ok(Self::new(id.unwrap()))
        }
    };
}

macro_rules! sql_impl_for_relation {
    ($rel_type:tt) => {
        impl<T: Entity<T, P>, P: PrimaryKey> ToSql for $rel_type<T, P> {
            fn to_sql(
                &self,
                ty: &Type,
                out: &mut BytesMut,
            ) -> Result<IsNull, Box<dyn Error + Sync + Send>>
            where
                Self: Sized,
            {
                self.target_id.to_sql(ty, out)
            }

            fn accepts(ty: &Type) -> bool
            where
                Self: Sized,
            {
                <u32 as ToSql>::accepts(ty)
            }

            fn to_sql_checked(
                &self,
                ty: &Type,
                out: &mut BytesMut,
            ) -> Result<IsNull, Box<dyn Error + Sync + Send>> {
                self.target_id.to_sql_checked(ty, out)
            }
        }

        impl<'a, T: Entity<T, P>, P: PrimaryKey + FromSql<'a>> FromSql<'a> for $rel_type<T, P> {
            fn from_sql(ty: &Type, raw: &'a [u8]) -> Result<Self, Box<dyn Error + Sync + Send>> {
                let id = P::from_sql(ty, raw)?;
                Ok($rel_type::<T, P>::new(id))
            }

            fn accepts(ty: &Type) -> bool {
                <P as FromSql>::accepts(ty)
            }
        }
    };
}

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
/// For the owning site of this relation, see [OneToOne].
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

/// Struct representing the many site of the n:1 relationship.
///
/// This struct holds the value of the relation.
///
/// The counterpart for [ManyToOne] is [OneToMany].
#[derive(Debug)]
pub struct ManyToOne<T: Entity<T, P>, P: PrimaryKey> {
    _p: PhantomData<T>,
    /// Raw id of the relation
    pub target_id: P,
}

impl<T: Entity<T, P>, P: PrimaryKey> ManyToOne<T, P> {
    default_relation_function!(ManyToOne);
}

sql_impl_for_relation!(ManyToOne);

/// Struct representing the one site of the n:1 relationship.
///
/// The counterpart for [OneToMany] is [ManyToOne].
///
/// Requires the mapped_by attribute to work as shown below.
/// ```
/// use crash_orm::{Entity, ManyToOne, OneToMany};
/// use crash_orm_derive::Entity;
///
/// #[derive(Entity, Debug)]
/// struct TestItem1 {
///     id: Option<u32>,
///     item2: ManyToOne<TestItem2, u32>,
/// }
///
/// #[derive(Entity, Debug)]
/// struct TestItem2 {
///     id: Option<u32>,
///     test: String,
///     #[mapped_by("item2")]
///     item1: OneToMany<TestItem1, u32>,
/// }
/// ```
#[derive(Debug)]
pub struct OneToMany<T: Entity<T, P>, P: PrimaryKey> {
    _p: PhantomData<T>,
    _p1: PhantomData<P>,
}

impl<T: Entity<T, P>, P: PrimaryKey> OneToMany<T, P> {
    /// Constructs a 1:n relation
    pub fn new() -> OneToMany<T, P> {
        OneToMany { _p: PhantomData, _p1: PhantomData }
    }
}
