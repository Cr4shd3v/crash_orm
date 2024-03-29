use crate::Entity;
use std::error::Error;
use std::fmt::Debug;
use std::marker::PhantomData;
use tokio_postgres::types::private::BytesMut;
use tokio_postgres::types::{FromSql, IsNull, ToSql, Type};

macro_rules! default_relation_function {
    ($rel_type:tt) => {
        pub const fn new(target_id: u32) -> $rel_type<T> {
            Self {
                _p: PhantomData,
                target_id,
            }
        }

        pub fn from(entity: &impl Entity<T>) -> crate::Result<$rel_type<T>> {
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
        impl<T: Entity<T>> ToSql for $rel_type<T> {
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

        impl<'a, T: Entity<T>> FromSql<'a> for $rel_type<T> {
            fn from_sql(ty: &Type, raw: &'a [u8]) -> Result<Self, Box<dyn Error + Sync + Send>> {
                Ok($rel_type::<T>::new(u32::from_sql(ty, raw)?))
            }

            fn accepts(ty: &Type) -> bool {
                <u32 as FromSql>::accepts(ty)
            }
        }
    };
}

#[derive(Debug)]
pub struct OneToOne<T: Entity<T>> {
    _p: PhantomData<T>,
    pub target_id: u32,
}

impl<T: Entity<T>> OneToOne<T> {
    default_relation_function!(OneToOne);
}

sql_impl_for_relation!(OneToOne);

#[derive(Debug)]
pub struct OneToOneRef<T: Entity<T>> {
    _p: PhantomData<T>,
}

impl<T: Entity<T>> OneToOneRef<T> {
    pub fn new() -> OneToOneRef<T> {
        OneToOneRef { _p: PhantomData }
    }
}

#[derive(Debug)]
pub struct ManyToOne<T: Entity<T>> {
    _p: PhantomData<T>,
    pub target_id: u32,
}

impl<T: Entity<T>> ManyToOne<T> {
    default_relation_function!(ManyToOne);
}

sql_impl_for_relation!(ManyToOne);

#[derive(Debug)]
pub struct OneToMany<T: Entity<T>> {
    _p: PhantomData<T>,
}

impl<T: Entity<T>> OneToMany<T> {
    pub fn new() -> OneToMany<T> {
        OneToMany { _p: PhantomData }
    }
}
