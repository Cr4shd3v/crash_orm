//! # Schema
//! The Schema trait is useful to manage the table itself with operations like create, delete or truncate.
//
//! This trait can be derived via macro.
//
//! Example:
//! ```rust
//! use crash_orm::derive::{Entity, Schema};
//!
//! #[derive(Entity, Debug, Schema)]
//! struct TestEntity {
//!     id: Option<u32>,
//! }
//! ```
//!
//! ## Check table exists
//! A simple function returning a bool whether a table exists or not.
//!
//! ```rust
//! # use crash_orm::derive::{Entity, Schema};
//! use crash_orm::Schema;
//! # use crash_orm_test::setup_test_connection;
//!
//! # #[derive(Entity, Debug, Schema)]
//! # struct TestEntity {
//! #    id: Option<u32>,
//! # }
//!
//! # tokio_test::block_on(async {
//! # let conn = setup_test_connection().await;
//! let exists: bool = TestEntity::table_exists(&conn).await.unwrap();
//! # });
//! ```
//!
//! ## Create Table
//! Creates the table based on the properties of the entity.
//!
//! ```no_run
//! # use crash_orm::derive::{Entity, Schema};
//! use crash_orm::Schema;
//! # use crash_orm_test::setup_test_connection;
//!
//! # #[derive(Entity, Debug, Schema)]
//! # struct TestEntity {
//! #    id: Option<u32>,
//! # }
//!
//! # tokio_test::block_on(async {
//! # let conn = setup_test_connection().await;
//! TestEntity::create_table(&conn).await.unwrap();
//! # });
//! ```
//!
//! ## Drop Table
//! Drop the table of the entity. This deletes the table itself from the database.
//!
//! ```no_run
//! # use crash_orm::derive::{Entity, Schema};
//! use crash_orm::Schema;
//! # use crash_orm_test::setup_test_connection;
//!
//! # #[derive(Entity, Debug, Schema)]
//! # struct TestEntity {
//! #    id: Option<u32>,
//! # }
//!
//! # tokio_test::block_on(async {
//! # let conn = setup_test_connection().await;
//! TestEntity::drop_table(&conn).await.unwrap();
//! # });
//! ```
//!
//! ## Truncate Table
//! Truncate the table of the entity. This deletes the content of the table from the database.
//!
//! ```no_run
//! # use crash_orm::derive::{Entity, Schema};
//! use crash_orm::Schema;
//! # use crash_orm_test::setup_test_connection;
//!
//! # #[derive(Entity, Debug, Schema)]
//! # struct TestEntity {
//! #    id: Option<u32>,
//! # }
//!
//! # tokio_test::block_on(async {
//! # let conn = setup_test_connection().await;
//! TestEntity::truncate_table(&conn).await.unwrap();
//! # });
//! ```

use async_trait::async_trait;

use crate::DatabaseConnection;

/// Trait implementing functions to modify the table itself in the database.
///
/// This trait can be derived, usually together with [Entity](crate::Entity).
///
/// ```
/// use crash_orm::derive::{Entity, Schema};
///
/// #[derive(Entity, Debug, Schema)]
/// struct TestItem {
///     id: Option<u32>,
///     name: String,
/// }
/// ```
#[async_trait]
pub trait Schema {
    /// Create the table based on the provided struct.
    ///
    /// This will fail, if the table already exists.
    /// Use [create_table_if_not_exists](Self::create_table_if_not_exists) if you want to ignore that.
    async fn create_table(connection: &impl DatabaseConnection) -> crate::Result<()>;

    /// Drop the table if it exists
    async fn drop_table(connection: &impl DatabaseConnection) -> crate::Result<()>;

    /// Empty the table
    async fn truncate_table(connection: &impl DatabaseConnection) -> crate::Result<()>;

    /// Check whether the table exists or not
    async fn table_exists(connection: &impl DatabaseConnection) -> crate::Result<bool>;

    /// Creates a table if it doesn't exist.
    ///
    /// This will not fail compared to [create_table](Self::create_table) if the table is already present.
    async fn create_table_if_not_exists(connection: &impl DatabaseConnection) -> crate::Result<()> {
        if !Self::table_exists(connection).await? {
            Self::create_table(connection).await?;
        }

        Ok(())
    }
}
