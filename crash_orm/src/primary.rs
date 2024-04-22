use std::fmt::Debug;

use tokio_postgres::types::{FromSql, ToSql};

pub trait PrimaryKey<'a>: Sync + Send + ToSql + FromSql<'a> + Debug + 'static {}

impl<'a> PrimaryKey<'a> for u32 {}

#[cfg(feature = "uuid")]
impl<'a> PrimaryKey<'a> for uuid::Uuid {}