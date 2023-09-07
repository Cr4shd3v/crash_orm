use std::error::Error;
use std::fmt::Debug;
use tokio_postgres::types::{FromSql, IsNull, ToSql, Type};
use tokio_postgres::types::private::BytesMut;
use crate::{DatabaseConnection, Entity};

macro_rules! default_relation_function {
    ($rel_type:tt) => {
        pub const fn new(target_id: u32) -> $rel_type<T> {
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

        pub fn from(entity: impl Entity<T>) -> crate::Result<$rel_type<T>> {
            let id = entity.get_id();
            if id.is_none() {
                return Err(crate::Error::from_str("Can't link an entity that hasn't been inserted yet"));
            }

            Ok(Self {
                target_id: id.unwrap(),
                value: None,
            })
        }
    };
}

macro_rules! sql_impl_for_relation {
    ($rel_type:tt) => {
        impl<T: Entity<T>> ToSql for $rel_type<T> {
            fn to_sql(&self, ty: &Type, out: &mut BytesMut) -> Result<IsNull, Box<dyn Error + Sync + Send>> where Self: Sized {
                self.target_id.to_sql(ty, out)
            }

            fn accepts(ty: &Type) -> bool where Self: Sized {
                <u32 as ToSql>::accepts(ty)
            }

            fn to_sql_checked(&self, ty: &Type, out: &mut BytesMut) -> Result<IsNull, Box<dyn Error + Sync + Send>> {
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
    target_id: u32,
    value: Option<T>,
}

impl<T: Entity<T>> OneToOne<T> {
    default_relation_function!(OneToOne);
}

sql_impl_for_relation!(OneToOne);

#[derive(Debug)]
pub struct ManyToOne<T: Entity<T>> {
    target_id: u32,
    value: Option<T>,
}

impl<T: Entity<T>> ManyToOne<T> {
    default_relation_function!(ManyToOne);
}

sql_impl_for_relation!(ManyToOne);