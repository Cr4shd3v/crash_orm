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

pub use column_definition::*;
pub use schema::*;
pub use table_definition::*;

mod schema;
mod column_definition;
mod table_definition;

