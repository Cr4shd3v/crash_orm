use std::error::Error;
use std::fmt::Debug;
use std::marker::PhantomData;

use tokio_postgres::types::{FromSql, IsNull, ToSql, Type};
use tokio_postgres::types::private::BytesMut;

use crate::{Entity, PrimaryKey};

macro_rules! default_relation_function {
    ($rel_type:tt) => {
        pub const fn new(target_id: P) -> $rel_type<T, P> {
            Self {
                _p: PhantomData,
                target_id,
            }
        }

        pub fn from(entity: &impl Entity<T, P>) -> crate::Result<$rel_type<T, P>> {
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

#[derive(Debug)]
pub struct OneToOne<T: Entity<T, P>, P: PrimaryKey> {
    _p: PhantomData<T>,
    pub target_id: P,
}

impl<T: Entity<T, P>, P: PrimaryKey> OneToOne<T, P> {
    default_relation_function!(OneToOne);
}

sql_impl_for_relation!(OneToOne);

#[derive(Debug)]
pub struct OneToOneRef<T: Entity<T, P>, P: PrimaryKey> {
    _p: PhantomData<T>,
    _p1: PhantomData<P>,
}

impl<T: Entity<T, P>, P: PrimaryKey> OneToOneRef<T, P> {
    pub fn new() -> OneToOneRef<T, P> {
        OneToOneRef { _p: PhantomData, _p1: PhantomData }
    }
}

#[derive(Debug)]
pub struct ManyToOne<T: Entity<T, P>, P: PrimaryKey> {
    _p: PhantomData<T>,
    pub target_id: P,
}

impl<T: Entity<T, P>, P: PrimaryKey> ManyToOne<T, P> {
    default_relation_function!(ManyToOne);
}

sql_impl_for_relation!(ManyToOne);

#[derive(Debug)]
pub struct OneToMany<T: Entity<T, P>, P: PrimaryKey> {
    _p: PhantomData<T>,
    _p1: PhantomData<P>,
}

impl<T: Entity<T, P>, P: PrimaryKey> OneToMany<T, P> {
    pub fn new() -> OneToMany<T, P> {
        OneToMany { _p: PhantomData, _p1: PhantomData }
    }
}
