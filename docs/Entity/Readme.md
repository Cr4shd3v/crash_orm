# Entity

## Declaration
To declare an entity you must do the following things:
- declare a struct
- add a field named "id" with the type Option\<u32>
  - despite being an option, the column resulting from id will not be nullable!
- derive Entity and Debug

Below is a minimal example:

```rust
use crash_orm::crash_orm_derive::Entity;

#[derive(Entity, Debug)]
struct TestItem {
    id: Option<u32>,
}
```

All properties declared in the struct will be treated as a column in the resulting table.

Take a look at all the [available types](Types.md).

If you want to link different entities via relation, take a look [here](Relations.md).

## Save Entities
There are a few functions on each entity to save them into the database.

### Insert Entity
To save a new entity you can use either insert_get_id or insert_set_id.

The difference between the two is simple:
- insert_get_id: Only requires an immutable reference to self and simply returns the id from the insertion
- insert_set_id: Requires a mutable reference to self and automatically sets the id on the entity in Rust

Example:

```rust
let mut entity = TestItem { id: None };
entity.insert_set_id(&conn).await?;

let entity2 = TestItem { id: None };
let id = entity.insert_get_id(&conn).await?;
```

### Update Entity
Updating an entity is way simpler, since there is only one function.

Example:

```rust
let entity = TestItem::get_by_id(&conn, 1).await?;
// Modify properties
entity.update(&conn).await?;
```

### Persist Entity
There is a way to do both insert and update depending on the entity.

If the id is not yet set, the persist function will call insert_set_id.
Because of that, the function requires a mutable reference to self.

If the id is set, the persist function will call update.

Example:

```rust
// Create or load entity

entity.persist(&conn).await?;
```

## Get Entity
Every entity has 2 simple functions to get one or many entities.

If you look for Queries instead, please check them out [here](../Query/Readme.md).

### Get by id
This simply returns the entity for the corresponding id from the database.
If there is no entry with the id, an error will be returned.

Example:
```rust
// Get the TestItem with the id 1
let entity = TestItem::get_by_id(&conn, 1).await?;
```

### Get all
You can also just get the entire table.

However, if you need to filter the results, you should use [Query](../Query/Readme.md) instead.

Example:
```rust
let all_entities = TestItem::get_all(&conn).await?;
```

## Remove Entity
To remove an entity, simply call remove on the corresponding entity:

```rust
let entity = TestItem::get_by_id(&conn, 1).await?;
entity.remove(&conn).await?;
```

NOTE: After this function call, the id of the entity will be empty! (This is why it needs &mut self).

## Functions for Vec\<Entity>
There are two utility functions to help with saving or deleting many entities.

### Persist Vec\<Entity>
You can save an entire vector of entities with just one function call:

```rust
let mut entities = vec![entity_1, /*...,*/ entity_n];
entities.persist_all(&conn).await?;
```

### Remove Vec\<Entity>
You can also remove an entire vector of entities:

```rust
let mut entities = vec![entity_1, /*...,*/ entity_n];
entities.remove_all(&conn).await?;
```

