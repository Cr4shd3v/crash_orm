use tokio_postgres::types::ToSql;

/// Trait marking a type as a primary key.
///
/// This trait **requires** for all implementations [Sync], [Send], [ToSql] and [FromSql](tokio_postgres::types::FromSql).
///
/// The trait is already implemented for [u32], [i32], [i64] and [Uuid](uuid::Uuid) (if the with-uuid feature is active)
pub trait PrimaryKey: Sync + Send + ToSql + 'static {}

impl PrimaryKey for u32 {}
impl PrimaryKey for i32 {}
impl PrimaryKey for i64 {}

#[cfg(feature = "uuid")]
impl PrimaryKey for uuid::Uuid {}