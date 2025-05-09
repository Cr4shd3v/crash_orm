//! # Entity
//!
//! **IMPORTANT NOTE**: You might need to refresh your cargo project in your IDE to see the generated code for auto complete.
//!
//! ## Declaration
//! To declare an entity you must do the following things:
//! - declare a struct
//! - add a field named "id"
//! - derive Entity and Debug
//!
//! Below is a minimal example:
//!
//! ```rust
//! use crash_orm::derive::Entity;
//!
//! #[derive(Entity, Debug)]
//! struct TestItem {
//!     id: u32,
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
//! To save a new entity you can use insert on the Create entity.
//!
//! Example:
//!
//! ```rust
//! use crash_orm::prelude::*;
//! # use crash_orm_test::setup_test_connection;
//!
//! # #[derive(Entity, Debug, Schema)]
//! # struct TestItemInsert {
//! #    id: u32,
//! # }
//!
//! # tokio_test::block_on(async {
//! # let conn = setup_test_connection().await;
//! # TestItemInsert::create_table_if_not_exists(&conn).await.unwrap();
//! let entity = TestItemInsertCreate {  }.insert(&conn).await.unwrap();
//!
//! let entity2 = TestItemInsertCreate {  };
//! let id = entity2.insert(&conn).await.unwrap();
//! # });
//! ```
//!
//! #### Exception: Uuid
//! When inserting an entity with uuid as primary key, **you** are responsible for generating the uuid.
//!
//! This is because there are many different versions of uuid, and you can choose which you want to use.
//!
//! However, you can choose to automatically generate uuid v4 or v7 for new entity through the feature flags `uuid-gen-v4` and `uuid-gen-v7`.
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
//! #    id: u32,
//! # }
//!
//! # tokio_test::block_on(async {
//! # let conn = setup_test_connection().await;
//! # TestItemUpdate::create_table_if_not_exists(&conn).await.unwrap();
//! # let entity2 = TestItemUpdateCreate {  }.insert(&conn).await.unwrap();
//! if let Some(entity) = TestItemUpdate::get_by_primary(&conn, entity2.id).await.unwrap() {
//!     // Modify entity properties
//!     entity.update(&conn).await.unwrap();
//! }
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
//! #    id: u32,
//! # }
//!
//! # tokio_test::block_on(async {
//! # let conn = setup_test_connection().await;
//! # TestItemGetById::create_table_if_not_exists(&conn).await.unwrap();
//! # let entity2 = TestItemGetByIdCreate {  }.insert(&conn).await.unwrap();
//! let entity = TestItemGetById::get_by_primary(&conn, entity2.id).await.unwrap();
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
//! #    id: u32,
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
//! #    id: u32,
//! # }
//!
//! # tokio_test::block_on(async {
//! # let conn = setup_test_connection().await;
//! # TestItemRemove::create_table_if_not_exists(&conn).await.unwrap();
//! # let entity2 = TestItemRemoveCreate {  }.insert(&conn).await.unwrap();
//! if let Some(entity) = TestItemRemove::get_by_primary(&conn, entity2.id).await.unwrap() {
//!     entity.remove(&conn).await.unwrap();
//! }
//! # });
//! ```
//!
//! ## Functions for Vec\<Entity>
//! There are two utility functions to help with saving or deleting many entities.
//!
//! ### Insert Vec\<Entity>
//! You can save an entire vector of entities with just one function call:
//!
//! ```rust
//! # use crash_orm::prelude::*;
//! # use crash_orm_test::setup_test_connection;
//!
//! # #[derive(Entity, Debug, Schema)]
//! # struct TestItemInsertVec {
//! #    id: u32,
//! #    test: i32,
//! # }
//!
//! # tokio_test::block_on(async {
//! # let conn = setup_test_connection().await;
//! # TestItemInsertVec::create_table_if_not_exists(&conn).await.unwrap();
//! # let entity_1 = TestItemInsertVecCreate { test: 1 };
//! # let entity_n = TestItemInsertVecCreate { test: 2 };
//! let entities = vec![entity_1, /*...,*/ entity_n];
//! entities.insert_all(&conn).await.unwrap();
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
//! #    id: u32,
//! # }
//!
//! # tokio_test::block_on(async {
//! # let conn = setup_test_connection().await;
//! # TestItem::create_table_if_not_exists(&conn).await.unwrap();
//! # let entity_1 = TestItemCreate {  };
//! # let entity_n = TestItemCreate {  };
//! let entities = TestItem::get_all(&conn).await.unwrap();
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
///     id: u32,
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

    /// Get all values of this create entity as vector for database insertion.
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
    async fn remove(&self, connection: &impl DatabaseConnection) -> Result<()>;

    /// Updates the entity in the database
    async fn update(&self, connection: &impl DatabaseConnection) -> Result<()>;

    /// Creates a SELECT [Query] for this entity.
    ///
    /// See [Query] for more details on how to build a query.
    fn query() -> Query<Self, Self, SelectQueryType> where Self: Sized {
        Query::new(BoxedSql::new(
            format!("SELECT * FROM public.{}", Self::TABLE_NAME),
            vec![],
        ))
    }

    /// Creates a DELETE [Query] for this entity.
    ///
    /// See [Query] for more details on how to build a query.
    fn delete() -> Query<Self, (), DeleteQueryType> where Self: Sized {
        Query::new(BoxedSql::new(
            format!("DELETE FROM public.{}", Self::TABLE_NAME),
            vec![],
        ))
    }

    /// Select specific columns ([EntityColumn] or [VirtualColumn]) from this entity.
    ///
    /// This returns a [SelectQuery]. See [SelectQuery] for more details.
    fn select_query<R: ResultMapping>(columns: &[&(dyn UntypedColumn<Self>)]) -> Query<Self, R, SelectQueryType> where Self: Sized {
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
            format!("SELECT {} FROM public.{}", query.join(","), Self::TABLE_NAME),
            values,
        ))
    }
}

/// Contains all primary key related functions of an entity.
#[async_trait]
pub trait PrimaryKeyEntity<P: ColumnType>: Entity {
    /// Returns the primary key of the entity.
    ///
    /// Used internally by the ORM
    fn get_primary(&self) -> P;

    /// Retrieves an entity by its primary key
    async fn get_by_primary(connection: &impl DatabaseConnection, id: P) -> Result<Option<Self>> where Self: Sized;
}

pub(crate) fn slice_query_value_iter<'a>(
    s: &'a [Arc<Box<dyn ToSql + Send + Sync>>],
) -> impl ExactSizeIterator<Item = &'a (dyn ToSql + Sync)> + 'a {
    s.iter().map(|s| &***s as _)
}

/// Trait implemented for all create structs for an entity
#[async_trait]
pub trait CreateEntity<E: Entity>: Sync + Send + 'static {
    /// Converts self into an actual entity.
    ///
    /// Also generates the [uuid::Uuid] if needed.
    fn into_entity(self) -> E;

    /// Calls [Self::into_entity] and inserts the new entity.
    async fn insert(self, connection: &impl DatabaseConnection) -> Result<E> where Self: Sized {
        let mut entity = self.into_entity();
        entity.insert(connection).await?;
        Ok(entity)
    }
}
