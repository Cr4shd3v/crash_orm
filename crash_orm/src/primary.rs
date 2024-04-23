use tokio_postgres::types::ToSql;

pub trait PrimaryKey: Sync + Send + ToSql + 'static {}

impl PrimaryKey for u32 {}
impl PrimaryKey for i32 {}
impl PrimaryKey for i64 {}

#[cfg(feature = "uuid")]
impl PrimaryKey for uuid::Uuid {}