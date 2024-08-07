//! Contains the [ColumnType] trait.

use postgres::types::FromSql;
use tokio_postgres::types::ToSql;

/// Trait marking a type as a valid column type.
///
/// This trait **requires** for all implementations [Sync], [Send], [ToSql] and [FromSql].
pub trait ColumnType: Sync + Send + ToSql + for<'a> FromSql<'a> + 'static {}

impl<T: Sync + Send + ToSql + for<'a> FromSql<'a> + 'static> ColumnType for T {}