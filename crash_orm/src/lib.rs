//! # Crash ORM
//!
//! Crash ORM is an async database ORM built for Postgres.
//!
//! The ORM is still in heavy development and bugs might occur.
//! There may also be breaking changes to the library in minor versions without further notice.
//!
//! # Documentation
//!- [Entity](entity)
//!  - [Property Types](column_value)
//!  - [Relations](relations)
//!- [Query](query)
//!- [Schema](schema)
//!- [Migration](migration)
//!
//! # Request changes for documentation
//! Please [open an issue](https://github.com/Cr4shd3v/crash_orm/issues/new/choose) with the "Documentation" Template.
//!
//! You can also directly open a pull request with the changes you propose.
//!
//! # Getting Started
//! This ORM only works with Postgres.
//! The ORM requires you to have a working installation of Postgres.
//!
//! To connect to postgres, you will need a connection string based on the following structure:
//!
//! postgresql://user:password@netloc/dbname
//!
//! # Create the CrashOrmDatabaseConnection
//! The first thing you want to do is creating the CrashOrmDatabaseConnection.
//!
//!```no_run
//! use crash_orm::postgres::NoTls;
//! use crash_orm::prelude::*;
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
//! For more info, visit the corresponding docs mentioned above.

#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![warn(missing_docs)]

pub extern crate async_trait;
pub extern crate crash_orm_derive as derive;
pub extern crate tokio_postgres as postgres;

pub use crate::error::*;

pub mod connection;
pub mod entity;
pub mod error;
pub mod entity_vec;
pub mod schema;
pub mod entity_column;
pub mod query_condition;
pub mod virtual_column;
pub mod column;
pub mod query;
pub mod column_value;
pub mod relations;
pub mod column_type;
#[cfg(feature = "migration")]
#[cfg_attr(docsrs, doc(cfg(feature = "migration")))]
pub mod migration;
pub mod boxed_sql;
mod result_mapping;

pub mod prelude {
    //! Reexports all required modules and crates

    pub use crate::async_trait::*;
    pub use crate::boxed_sql::*;
    pub use crate::column::*;
    pub use crate::column_type::*;
    pub use crate::column_value::*;
    pub use crate::connection::*;
    pub use crate::derive::*;
    pub use crate::entity::*;
    pub use crate::entity_column::*;
    pub use crate::entity_vec::*;
    pub use crate::error::*;
    #[cfg(feature = "migration")]
    #[cfg_attr(docsrs, doc(cfg(feature = "migration")))]
    pub use crate::migration::*;
    pub use crate::query::*;
    pub use crate::query_condition::*;
    pub use crate::relations::*;
    pub use crate::schema::*;
    pub use crate::virtual_column::*;

    pub extern crate tokio_postgres as postgres;
}