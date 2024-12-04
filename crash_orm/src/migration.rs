//! # Migration
//! 
//! ## Setup
//! First, create a new lib crate in your project folder.
//! ```shell
//! cargo new migration --lib
//! ```
//! 
//! Next, add crash_orm as dependency for this new crate.
//! 
//! Inside the lib.rs, create your migration manager:
//! 
//! ```
//! use crash_orm::prelude::*;
//! 
//! pub struct MigrationManager;
//! 
//! impl CrashOrmMigrationManager for MigrationManager {
//!     fn get_migrations() -> Vec<Box<dyn Migration>> {
//!         vec![
//!             
//!         ]
//!     }
//! }
//! ```
//! 
//! ## Create Migration
//! 
//! As of now you have to manually create migrations. However, it is planned to generate them in the future.
//! 
//! ```
//! use crash_orm::async_trait::async_trait;
//! use crash_orm::prelude::{CrashOrmDatabaseConnection, DatabaseConnection, Schema};
//! use crash_orm::migration::Migration;
//!
//! pub struct ExampleMigration;
//!
//! #[async_trait]
//! impl Migration for ExampleMigration {
//!     async fn up(&self, conn: &CrashOrmDatabaseConnection) -> crash_orm::Result<()> {
//!         // UP, like User::create_table_if_not_exists(conn).await?;
//!         Ok(())
//!     }
//!
//!     async fn down(&self, conn: &CrashOrmDatabaseConnection) -> crash_orm::Result<()> {
//!         // DOWN, like User::drop_table(conn).await?;
//!         Ok(())
//!     }
//!
//!     fn get_name(&self) -> &str {
//!         "ExampleMigration" // The name MUST ALWAYS be UNIQUE
//!     }
//! }
//! ```
//! 
//! IMPORTANT: The returned string from get_name MUST be UNIQUE across ALL migrations.
//! 
//! ## Execute Migrations
//! On Startup of your app, you should call the migrate_up method of your migration manager:
//! 
//! ```
//! use crash_orm::prelude::*;
//! # use crash_orm_test::setup_test_connection;
//!
//! pub struct MigrationManager;
//!
//! impl CrashOrmMigrationManager for MigrationManager {
//!     fn get_migrations() -> Vec<Box<dyn Migration>> {
//!         vec![
//!
//!         ]
//!     }
//! }
//! # tokio_test::block_on(async {
//! # let conn = setup_test_connection().await;
//! MigrationManager::migrate_up(&conn).await.unwrap();
//! # });
//! ```
//! 
//! You should take care of the potential Err returned by this function since this likely means that parts of your migration failed.
//! 
//! migrate_up terminates after the first error, no following statements are executed.

pub use entity::*;
pub use migration::*;
pub use migration_manager::*;

mod entity;
mod migration;
mod migration_manager;

