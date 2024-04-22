use std::error::Error;
use std::fmt::Debug;
use std::marker::PhantomData;

use tokio_postgres::types::{FromSql, IsNull, ToSql, Type};
use tokio_postgres::types::private::BytesMut;

use crate::{Entity, PrimaryKey};

macro_rules! default_relation_function {
    ($rel_type:tt) => {
        pub const fn new(target_id: PRIMARY) -> $rel_type<T, PRIMARY> {
            Self {
                _p: PhantomData,
                target_id,
            }
        }

        pub fn from(entity: &impl Entity<T, PRIMARY>) -> crate::Result<$rel_type<T, PRIMARY>> {
            let id = entity.get_id();
            if id.is_none() {
                return Err(crate::Error::from_str(
                    "Can't link an entity that hasn't been inserted yet",
                ));
            }

            Ok(Self {
                _p: PhantomData,
                target_id: id.unwrap(),
            })
        }
    };
}

macro_rules! sql_impl_for_relation {
    ($rel_type:tt) => {
        impl<T: Entity<T, PRIMARY>, PRIMARY: PrimaryKey<'static>> ToSql for $rel_type<T, PRIMARY> {
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

        impl<T: Entity<T, PRIMARY>, PRIMARY: PrimaryKey<'static>> FromSql<'static> for $rel_type<T, PRIMARY> {
            fn from_sql(ty: &Type, raw: &'static [u8]) -> Result<Self, Box<dyn Error + Sync + Send>> {
                Ok($rel_type::<T, PRIMARY>::new(PRIMARY::from_sql(ty, raw)?))
            }

            fn accepts(ty: &Type) -> bool {
                <u32 as FromSql>::accepts(ty)
            }
        }
    };
}

#[derive(Debug)]
pub struct OneToOne<T: Entity<T, PRIMARY>, PRIMARY: PrimaryKey<'static>> {
    _p: PhantomData<T>,
    pub target_id: PRIMARY,
}

impl<T: Entity<T, PRIMARY>, PRIMARY: PrimaryKey<'static>> OneToOne<T, PRIMARY> {
    default_relation_function!(OneToOne);
}

sql_impl_for_relation!(OneToOne);

#[derive(Debug)]
pub struct OneToOneRef<T: Entity<T, PRIMARY>, PRIMARY: PrimaryKey<'static>> {
    _p: PhantomData<T>,
    _p1: PhantomData<PRIMARY>,
}

impl<T: Entity<T, PRIMARY>, PRIMARY: PrimaryKey<'static>> OneToOneRef<T, PRIMARY> {
    pub fn new() -> OneToOneRef<T, PRIMARY> {
        OneToOneRef { _p: PhantomData, _p1: PhantomData }
    }
}

#[derive(Debug)]
pub struct ManyToOne<T: Entity<T, PRIMARY>, PRIMARY: PrimaryKey<'static>> {
    _p: PhantomData<T>,
    pub target_id: PRIMARY,
}

impl<T: Entity<T, PRIMARY>, PRIMARY: PrimaryKey<'static>> ManyToOne<T, PRIMARY> {
    default_relation_function!(ManyToOne);
}

sql_impl_for_relation!(ManyToOne);

#[derive(Debug)]
pub struct OneToMany<T: Entity<T, PRIMARY>, PRIMARY: PrimaryKey<'static>> {
    _p: PhantomData<T>,
    _p1: PhantomData<PRIMARY>,
}

impl<T: Entity<T, PRIMARY>, PRIMARY: PrimaryKey<'static>> OneToMany<T, PRIMARY> {
    pub fn new() -> OneToMany<T, PRIMARY> {
        OneToMany { _p: PhantomData, _p1: PhantomData }
    }
}
