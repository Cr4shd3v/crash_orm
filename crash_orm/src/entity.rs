//! # Entity
//!
//! **IMPORTANT NOTE**: You might need to refresh your cargo project in your IDE to see the generated code for auto complete.
//!
//! ## Declaration
//! To declare an entity you must do the following things:
//! - declare a struct
//! - add a field named "id" with the type Option\<u32>
//!   - despite being an option, the column resulting from id will not be nullable!
//! - derive Entity and Debug
//!
//! Below is a minimal example:
//!
//! ```rust
//! use crash_orm::derive::Entity;
//!
//! #[derive(Entity, Debug)]
//! struct TestItem {
//!     id: Option<u32>,
//! }
//! ```
//!
//! All properties declared in the struct will be treated as a column in the resulting table.
//!
//! Take a look at all the [available types](Types.md).
//!
//! If you want to link different entities via relation, take a look [here](Relations.md).
//!
//! ## Save Entities
//! There are a few functions on each entity to save them into the database.
//!
//! ### Insert Entity
//! To save a new entity you can use either insert_get_id or insert_set_id.
//!
//! The difference between the two is simple:
//! - insert_get_id: Only requires an immutable reference to self and simply returns the id from the insertion
//! - insert_set_id: Requires a mutable reference to self and automatically sets the id on the entity in Rust
//!
//! Example:
//!
//! ```rust
//! use crash_orm::prelude::*;
//! # use crash_orm_test::setup_test_connection;
//!
//! # #[derive(Entity, Debug, Schema)]
//! # struct TestItemInsert {
//! #    id: Option<u32>,
//! # }
//!
//! # tokio_test::block_on(async {
//! # let conn = setup_test_connection().await;
//! # TestItemInsert::create_table_if_not_exists(&conn).await.unwrap();
//! let mut entity = TestItemInsert { id: None };
//! entity.insert_set_id(&conn).await.unwrap();
//!
//! let entity2 = TestItemInsert { id: None };
//! let id = entity.insert_get_id(&conn).await.unwrap();
//! # });
//! ```
//!
//! #### Exception: Uuid
//! When inserting an entity with a uuid as primary key, **you** are responsible for generating the uuid.
//!
//! This is because there are many different versions of uuid, and you can choose which you want to use.
//!
//! ### Update Entity
//! Updating an entity is way simpler, since there is only one function.
//!
//! Example:
//! ```rust
//! use crash_orm::prelude::*;
//! # use crash_orm_test::setup_test_connection;
//!
//! # #[derive(Entity, Debug, Schema)]
//! # struct TestItemUpdate {
//! #    id: Option<u32>,
//! # }
//!
//! # tokio_test::block_on(async {
//! # let conn = setup_test_connection().await;
//! # TestItemUpdate::create_table_if_not_exists(&conn).await.unwrap();
//! # let entity2 = TestItemUpdate { id: None };
//! # let id = entity2.insert_get_id(&conn).await.unwrap();
//! let entity = TestItemUpdate::get_by_primary(&conn, id).await.unwrap();
//! // Modify entity properties
//! entity.update(&conn).await.unwrap();
//! # });
//! ```
//!
//! ### Persist Entity
//! There is a way to do both insert and update depending on the entity.
//!
//! If the id is not yet set, the persist function will call insert_set_id.
//! Because of that, the function requires a mutable reference to self.
//!
//! If the id is set, the persist function will call update.
//!
//! Example:
//! ```rust
//! use crash_orm::prelude::*;
//! # use crash_orm_test::setup_test_connection;
//!
//! # #[derive(Entity, Debug, Schema)]
//! # struct TestItemPersist {
//! #    id: Option<u32>,
//! # }
//!
//! # tokio_test::block_on(async {
//! # let conn = setup_test_connection().await;
//! # TestItemPersist::create_table_if_not_exists(&conn).await.unwrap();
//! # let entity2 = TestItemPersist { id: None };
//! # let id = entity2.insert_get_id(&conn).await.unwrap();
//! let mut entity = TestItemPersist::get_by_primary(&conn, id).await.unwrap();
//! // Modify entity properties
//! entity.persist(&conn).await.unwrap();
//! # });
//! ```
//!
//! ## Get Entity
//! Every entity has 2 simple functions to get one or many entities.
//!
//! If you look for Queries instead, please check them out [here](../Query/Readme.md).
//!
//! ### Get by id
//! This simply returns the entity for the corresponding id from the database.
//! If there is no entry with the id, an error will be returned.
//!
//! Example:
//! ```rust
//! use crash_orm::prelude::*;
//! # use crash_orm_test::setup_test_connection;
//!
//! # #[derive(Entity, Debug, Schema)]
//! # struct TestItemGetById {
//! #    id: Option<u32>,
//! # }
//!
//! # tokio_test::block_on(async {
//! # let conn = setup_test_connection().await;
//! # TestItemGetById::create_table_if_not_exists(&conn).await.unwrap();
//! # let entity2 = TestItemGetById { id: None };
//! # let id = entity2.insert_get_id(&conn).await.unwrap();
//! let entity = TestItemGetById::get_by_primary(&conn, id).await.unwrap();
//! # });
//! ```
//!
//! ### Get all
//! You can also just get the entire table.
//!
//! However, if you need to filter the results, you should use [Query](../Query/Readme.md) instead.
//!
//! Example:
//! ```rust
//! use crash_orm::prelude::*;
//! # use crash_orm_test::setup_test_connection;
//!
//! # #[derive(Entity, Debug, Schema)]
//! # struct TestItemGetAll {
//! #    id: Option<u32>,
//! # }
//!
//! # tokio_test::block_on(async {
//! # let conn = setup_test_connection().await;
//! # TestItemGetAll::create_table_if_not_exists(&conn).await.unwrap();
//! let entity = TestItemGetAll::get_all(&conn).await.unwrap();
//! # });
//! ```
//!
//! ## Remove Entity
//! To remove an entity, simply call remove on the corresponding entity:
//!
//! ```rust
//! use crash_orm::prelude::*;
//! # use crash_orm_test::setup_test_connection;
//!
//! # #[derive(Entity, Debug, Schema)]
//! # struct TestItemRemove {
//! #    id: Option<u32>,
//! # }
//!
//! # tokio_test::block_on(async {
//! # let conn = setup_test_connection().await;
//! # TestItemRemove::create_table_if_not_exists(&conn).await.unwrap();
//! # let entity2 = TestItemRemove { id: None };
//! # let id = entity2.insert_get_id(&conn).await.unwrap();
//! let mut entity = TestItemRemove::get_by_primary(&conn, id).await.unwrap();
//! entity.remove(&conn).await.unwrap();
//! # });
//! ```
//!
//! NOTE: After this function call, the id of the entity will be empty! (This is why it needs &mut self).
//!
//! ## Functions for Vec\<Entity>
//! There are two utility functions to help with saving or deleting many entities.
//!
//! ### Persist Vec\<Entity>
//! You can save an entire vector of entities with just one function call:
//!
//! ```rust
//! # use crash_orm::prelude::*;
//! # use crash_orm_test::setup_test_connection;
//!
//! # #[derive(Entity, Debug, Schema)]
//! # struct TestItemPersist {
//! #    id: Option<u32>,
//! # }
//!
//! # tokio_test::block_on(async {
//! # let conn = setup_test_connection().await;
//! # TestItemPersist::create_table_if_not_exists(&conn).await.unwrap();
//! # let entity_1 = TestItemPersist { id: None };
//! # let entity_n = TestItemPersist { id: None };
//! let mut entities = vec![entity_1, /*...,*/ entity_n];
//! entities.persist_all(&conn).await.unwrap();
//! # });
//! ```
//!
//! ### Remove Vec\<Entity>
//! You can also remove an entire vector of entities:
//!
//! ```rust
//! use crash_orm::prelude::*;
//! # use crash_orm_test::setup_test_connection;
//!
//! # #[derive(Entity, Debug, Schema)]
//! # struct TestItem {
//! #    id: Option<u32>,
//! # }
//!
//! # tokio_test::block_on(async {
//! # let conn = setup_test_connection().await;
//! # TestItem::create_table_if_not_exists(&conn).await.unwrap();
//! # let entity_1 = TestItem { id: None };
//! # let entity_n = TestItem { id: None };
//! let mut entities = vec![entity_1, /*...,*/ entity_n];
//! # entities = TestItem::get_all(&conn).await.unwrap();
//! entities.remove_all(&conn).await.unwrap();
//! # });
//! ```
use std::fmt::Debug;
use std::sync::Arc;

use async_trait::async_trait;
use tokio_postgres::types::ToSql;

use crate::prelude::*;
use crate::result_mapping::ResultMapping;

/// Trait implemented for all database entities.
///
/// This trait can be derived, but the structs **have to** derive [Debug] as well.
/// ```
/// use crash_orm::derive::Entity;
///
/// #[derive(Entity, Debug)]
/// struct TestEntity {
///     id: Option<u32>,
///     name: String,
/// }
/// ```
#[async_trait]
pub trait Entity: ResultMapping + Send + Sync + Debug + 'static {
    /// Name of the table
    const TABLE_NAME: &'static str;

    /// Internal field for batch insert
    #[doc(hidden)]
    const __INSERT_FIELD_NAMES: &'static str;

    /// This type references the column struct of this entity
    type ColumnType;

    /// Get all values of this entity as vector for database insertion.
    ///
    /// This method is used internally and should not be used manually.
    #[doc(hidden)]
    fn get_values(&self) -> Vec<&(dyn ToSql + Sync)>;

    /// Retrieves all entities
    async fn get_all(connection: &impl DatabaseConnection) -> Result<Vec<Self>> where Self: Sized;

    /// Returns the count of entries in the table
    async fn count(connection: &impl DatabaseConnection) -> Result<i64>;

    /// Insert and set id
    ///
    /// This sets the id in the entity
    async fn insert(&mut self, connection: &impl DatabaseConnection) -> Result<()>;

    /// Removes the entity from the database
    async fn remove(&mut self, connection: &impl DatabaseConnection) -> Result<()>;

    /// Updates the entity in the database
    async fn update(&self, connection: &impl DatabaseConnection) -> Result<()>;

    /// Persist this entity.
    ///
    /// If the entity is not yet inserted, [`Self::insert_set_id`] is called.
    /// If the entity is already inserted, [`Self::update`] is called.
    async fn persist(&mut self, connection: &impl DatabaseConnection) -> Result<()>;

    /// Creates a [Query] for this Entity.
    ///
    /// See [Query] for more details on how to build a query.
    fn query() -> Query<Self, Self> where Self: Sized {
        Query::new(BoxedSql::new(
            format!("SELECT * FROM {}", Self::TABLE_NAME),
            vec![],
        ))
    }

    /// Select specific columns ([EntityColumn] or [VirtualColumn]) from this entity.
    ///
    /// This returns a [SelectQuery]. See [SelectQuery] for more details.
    fn select_query<R: ResultMapping>(columns: &[&(dyn UntypedColumn<Self>)]) -> Query<Self, R> where Self: Sized {
        let columns = columns
            .iter()
            .map(|v| v.get_sql())
            .collect::<Vec<BoxedSql>>();
        let mut query = vec![];
        let mut values = vec![];
        let mut index = 1;

        for column in columns {
            let (new_query, new_values, next_index) = column.resolve(index);
            query.push(new_query);
            values.extend(new_values);
            index = next_index;
        }

        Query::new(BoxedSql::new(
            format!("SELECT {} FROM {}", query.join(","), Self::TABLE_NAME),
            values,
        ))
    }
}

/// Contains all primary key related functions of an entity.
#[async_trait]
pub trait PrimaryKeyEntity<P: ColumnType>: Entity {
    /// Returns the id of the entity.
    ///
    /// Used internally by the ORM
    fn get_primary(&self) -> Option<P>;

    /// Retrieves an entity by its primary key
    async fn get_by_primary(connection: &impl DatabaseConnection, id: P) -> Result<Self> where Self: Sized;
}

pub(crate) fn slice_query_value_iter<'a>(
    s: &'a [Arc<Box<dyn ToSql + Send + Sync>>],
) -> impl ExactSizeIterator<Item = &'a (dyn ToSql + Sync)> + 'a {
    s.iter().map(|s| &***s as _)
}
