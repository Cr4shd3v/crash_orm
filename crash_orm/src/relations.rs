use std::error::Error;
use std::fmt::Debug;
use tokio_postgres::types::{FromSql, IsNull, ToSql, Type};
use tokio_postgres::types::private::BytesMut;
use crate::{DatabaseConnection, Entity};

macro_rules! default_get_function {
    () => {
        pub async fn get(&mut self, conn: &DatabaseConnection) -> crate::Result<&T> {
            if self.value.is_none() {
                self.value = Some(T::get_by_id(&conn, self.target_id).await?);
            }

            Ok(self.value.as_ref().unwrap())
        }
    };
}

#[derive(Debug)]
pub struct OneToOne<T: Entity<T>> {
    target_id: u32,
    value: Option<T>,
}

impl<T: Entity<T>> OneToOne<T> {
    pub const fn new(target_id: u32) -> OneToOne<T> {
        Self {
            target_id,
            value: None,
        }
    }

    default_get_function!();
}

impl<T: Entity<T>> ToSql for OneToOne<T> {
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

impl<'a, T: Entity<T>> FromSql<'a> for OneToOne<T> {
    fn from_sql(ty: &Type, raw: &'a [u8]) -> Result<Self, Box<dyn Error + Sync + Send>> {
        Ok(OneToOne::<T>::new(u32::from_sql(ty, raw)?))
    }

    fn accepts(ty: &Type) -> bool {
        <u32 as FromSql>::accepts(ty)
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

    default_get_function!();
}