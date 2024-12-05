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
            Ok(Self::new(id))
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

        #[cfg(feature = "serialize")]
        impl<'a, T: PrimaryKeyEntity<P>, P: ColumnType + serde::Deserialize<'a>> serde::Deserialize<'a> for $rel_type<T, P> {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'a>
            {
                P::deserialize(deserializer).map(|v| $rel_type::new(v))
            }
        }

        #[cfg(feature = "serialize")]
        impl<T: PrimaryKeyEntity<P>, P: ColumnType + serde::Serialize> serde::Serialize for $rel_type<T, P> {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer
            {
                P::serialize(&self.target_id, serializer)
            }
        }
    };
}

pub(super) use default_relation_function;
pub(super) use sql_impl_for_relation;