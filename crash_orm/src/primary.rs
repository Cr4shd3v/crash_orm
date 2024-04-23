use tokio_postgres::types::{FromSql, ToSql};

pub trait PrimaryKey: Sync + Send + ToSql + FromSql<'static> + 'static {}

impl PrimaryKey for u32 {}
impl PrimaryKey for i32 {}
impl PrimaryKey for i64 {}

#[cfg(feature = "uuid")]
impl PrimaryKey for uuid::Uuid {}