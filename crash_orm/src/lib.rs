//! # Crash ORM
//!
//! A simple, typed ORM for Postgres.
//!
//! # Getting Started
//!
//! # Setup Postgres
//! This ORM only works with Postgres.
//! The ORM requires you to have a working installation of Postgres.
//!
//! To connect to postgres, you will need a connection string based on the following structure:
//!
//! ## postgresql://user:password@netloc/dbname
//!
//!
//! # Create the CrashOrmDatabaseConnection
//! The first thing you want to do is creating the CrashOrmDatabaseConnection.
//!
//! ```no_run
//! use crash_orm::CrashOrmDatabaseConnection;
//! use crash_orm::postgres::NoTls;
//!
//! # tokio_test::block_on(async {
//! let conn = CrashOrmDatabaseConnection::new("postgresql://<user>:<password>@<netloc>/<dbname>", NoTls).await
//!     .expect("Failed to connect to database");
//! # })
//! ```
//!
//! This connection is mandatory for all functions on entities.
//!
//! You should store this variable globally or in case of a web framework like actix you can add it as web::Data.
//!
//! # Your first Entity
//! You can now declare your first Entity.
//!
//! ```
//! use crash_orm::derive::{Entity, Schema};
//!
//! #[derive(Debug, Entity, Schema)]
//! struct Person {
//!     id: Option<u32>,
//!     name: String,
//! }
//! ```
//!
//! This will generate a lot of code, if you are curious, you can inspect this struct with 'cargo expand'.
//!
//! For more info, visit the [repository](https://github.com/Cr4shd3v/crash_orm).

#![warn(missing_docs)]

pub extern crate async_trait;
pub extern crate crash_orm_derive as derive;
pub extern crate tokio_postgres as postgres;

pub use column::*;
pub use column_value::*;
pub use connection::*;
pub use entity::*;
pub use entity_column::*;
pub use entity_vec::*;
pub use error::*;
pub use primary::*;
pub use query::*;
pub use query_condition::*;
pub use relations::*;
pub use schema::*;
pub use virtual_column::*;

mod connection;
mod entity;
mod error;
mod entity_vec;
mod schema;
mod entity_column;
mod query_condition;
mod virtual_column;
mod column;
mod query;
mod column_value;
mod relations;
mod primary;
