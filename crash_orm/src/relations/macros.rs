#[allow(missing_docs)]
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
        /// This utilizes the [Entity::get_primary] function.
        pub fn from(entity: &impl PrimaryKeyEntity<P>) -> crate::Result<$rel_type<T, P>> {
            let id = entity.get_primary();
            if id.is_none() {
                return Err(crate::Error::from_str(
                    "Can't link an entity that hasn't been inserted yet",
                ));
            }

            Ok(Self::new(id.unwrap()))
        }
    };
}

#[allow(missing_docs)]
macro_rules! sql_impl_for_relation {
    ($rel_type:tt) => {
        impl<T: PrimaryKeyEntity<P>, P: ColumnType> tokio_postgres::types::ToSql for $rel_type<T, P> {
            fn to_sql(
                &self,
                ty: &tokio_postgres::types::Type,
                out: &mut postgres::types::private::BytesMut,
            ) -> Result<tokio_postgres::types::IsNull, Box<dyn std::error::Error + Sync + Send>>
            where
                Self: Sized,
            {
                self.target_id.to_sql(ty, out)
            }

            fn accepts(ty: &tokio_postgres::types::Type) -> bool
            where
                Self: Sized,
            {
                <P as tokio_postgres::types::ToSql>::accepts(ty)
            }

            fn to_sql_checked(
                &self,
                ty: &tokio_postgres::types::Type,
                out: &mut postgres::types::private::BytesMut,
            ) -> Result<tokio_postgres::types::IsNull, Box<dyn std::error::Error + Sync + Send>> {
                self.target_id.to_sql_checked(ty, out)
            }
        }

        impl<'a, T: PrimaryKeyEntity<P>, P: ColumnType> tokio_postgres::types::FromSql<'a> for $rel_type<T, P> {
            fn from_sql(ty: &tokio_postgres::types::Type, raw: &'a [u8]) -> Result<Self, Box<dyn std::error::Error + Sync + Send>> {
                let id = P::from_sql(ty, raw)?;
                Ok($rel_type::<T, P>::new(id))
            }

            fn accepts(ty: &tokio_postgres::types::Type) -> bool {
                <P as tokio_postgres::types::FromSql>::accepts(ty)
            }
        }
    };
}

pub(super) use default_relation_function;
pub(super) use sql_impl_for_relation;